//! CPU mip chains for the power-of-two RGBA8 procedural surface maps.
use bevy::{
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};

pub(super) fn filter_generated_map(image: &mut Image) {
    let mut width = image.width() as usize;
    let mut height = image.height() as usize;
    assert!(width.is_power_of_two() && height.is_power_of_two());
    let srgb = image.texture_descriptor.format.is_srgb();
    let data = image.data.as_mut().expect("generated surface pixels");
    assert_eq!(data.len(), width * height * 4);
    let mut offset = 0;
    let mut levels = 1;
    while width > 1 || height > 1 {
        let next_width = (width / 2).max(1);
        let next_height = (height / 2).max(1);
        let next_offset = data.len();
        for y in 0..next_height {
            for x in 0..next_width {
                for channel in 0..4 {
                    let mut sum = 0.;
                    for dy in 0..2 {
                        for dx in 0..2 {
                            let sx = (x * 2 + dx).min(width - 1);
                            let sy = (y * 2 + dy).min(height - 1);
                            let value =
                                data[offset + (sy * width + sx) * 4 + channel] as f32 / 255.;
                            sum += if srgb && channel < 3 {
                                Srgba::gamma_function(value)
                            } else {
                                value
                            };
                        }
                    }
                    let value = sum * 0.25;
                    let value = if srgb && channel < 3 {
                        Srgba::gamma_function_inverse(value)
                    } else {
                        value
                    };
                    data.push((value * 255.).round() as u8);
                }
            }
        }
        offset = next_offset;
        width = next_width;
        height = next_height;
        levels += 1;
    }
    image.texture_descriptor.mip_level_count = levels;
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 8,
        ..default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{
        asset::RenderAssetUsages,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    };

    #[test]
    fn mip_chain_filters_color_in_linear_light_and_keeps_data_maps_linear() {
        for (format, expected) in [
            (TextureFormat::Rgba8UnormSrgb, 188),
            (TextureFormat::Rgba8Unorm, 128),
        ] {
            let mut image = Image::new_fill(
                Extent3d {
                    width: 4,
                    height: 4,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &[0, 0, 0, 255, 255, 255, 255, 255],
                format,
                RenderAssetUsages::default(),
            );
            let original = image.data.clone().unwrap();
            filter_generated_map(&mut image);
            let data = image.data.as_ref().unwrap();
            assert_eq!(image.texture_descriptor.mip_level_count, 3);
            assert_eq!(data.len(), (16 + 4 + 1) * 4);
            assert_eq!(&data[..64], original.as_slice());
            for pixel in data[64..].chunks_exact(4) {
                assert_eq!(pixel, [expected, expected, expected, 255]);
            }
            let ImageSampler::Descriptor(sampler) = &image.sampler else {
                panic!("explicit surface sampler");
            };
            assert_eq!(sampler.min_filter, ImageFilterMode::Linear);
            assert_eq!(sampler.mag_filter, ImageFilterMode::Linear);
            assert_eq!(sampler.mipmap_filter, ImageFilterMode::Linear);
            assert_eq!(sampler.anisotropy_clamp, 8);
        }
    }
}

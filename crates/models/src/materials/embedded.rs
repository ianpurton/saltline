//! Small authored surface set, embedded so copied native executables work offline.
use bevy::{
    asset::RenderAssetUsages,
    image::{
        CompressedImageFormats, ImageAddressMode, ImageFilterMode, ImageSampler,
        ImageSamplerDescriptor, ImageType,
    },
    prelude::*,
};
pub fn png(images: &mut Assets<Image>, bytes: &[u8], srgb: bool) -> Handle<Image> {
    let mut result = Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        srgb,
        ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            mag_filter: ImageFilterMode::Linear,
            min_filter: ImageFilterMode::Linear,
            mipmap_filter: ImageFilterMode::Linear,
            anisotropy_clamp: 8,
            ..default()
        }),
        RenderAssetUsages::default(),
    )
    .expect("bundled surface PNG must decode");
    // Authored maps need mipmaps to avoid sparkling at grazing angles.
    let mut source = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .expect("bundled PNG")
        .to_rgba8();
    let mut data = source.as_raw().clone();
    let mut levels = 1;
    while source.width() > 1 || source.height() > 1 {
        source = image::imageops::resize(
            &source,
            (source.width() / 2).max(1),
            (source.height() / 2).max(1),
            image::imageops::FilterType::Triangle,
        );
        data.extend_from_slice(source.as_raw());
        levels += 1;
    }
    result.data = Some(data);
    result.texture_descriptor.mip_level_count = levels;
    images.add(result)
}
pub fn surfaces(images: &mut Assets<Image>) -> [(Handle<Image>, Handle<Image>, Handle<Image>); 2] {
    [
        (
            png(
                images,
                include_bytes!("../../../../assets/materials/asphalt-color.png"),
                true,
            ),
            png(
                images,
                include_bytes!("../../../../assets/materials/asphalt-normal.png"),
                false,
            ),
            png(
                images,
                include_bytes!("../../../../assets/materials/asphalt-arm.png"),
                false,
            ),
        ),
        (
            png(
                images,
                include_bytes!("../../../../assets/materials/concrete-color.png"),
                true,
            ),
            png(
                images,
                include_bytes!("../../../../assets/materials/concrete-normal.png"),
                false,
            ),
            png(
                images,
                include_bytes!("../../../../assets/materials/concrete-arm.png"),
                false,
            ),
        ),
    ]
}

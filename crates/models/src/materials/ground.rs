//! Reference-generated, world-aligned ground data shared by shading and surface queries.
use bevy::{
    asset::RenderAssetUsages,
    image::{CompressedImageFormats, ImageAddressMode, ImageSampler, ImageType},
    prelude::*,
};

pub const MASK_ORIGIN: Vec2 = Vec2::new(-44., -40.);
pub const MASK_SIZE: f32 = 80.;

#[derive(Resource, Clone)]
pub struct GroundMaps {
    pub mask: Handle<Image>,
    pub marks: Handle<Image>,
    pub detail: Handle<Image>,
    pub normal: Handle<Image>,
    pixels: Vec<u8>,
    resolution: usize,
}
impl GroundMaps {
    pub fn new(images: &mut Assets<Image>) -> Self {
        let mut load = |bytes: &[u8], repeat: bool| {
            let mut image = Image::from_buffer(
                bytes,
                ImageType::Extension("png"),
                CompressedImageFormats::NONE,
                false,
                ImageSampler::default(),
                RenderAssetUsages::default(),
            )
            .expect("bundled reference ground map");
            super::mipmaps::filter_generated_map(&mut image);
            if !repeat {
                if let ImageSampler::Descriptor(sampler) = &mut image.sampler {
                    sampler.address_mode_u = ImageAddressMode::ClampToEdge;
                    sampler.address_mode_v = ImageAddressMode::ClampToEdge;
                }
            }
            images.add(image)
        };
        let mask = load(include_bytes!("../../../../assets/ground/mask.png"), false);
        let marks = load(include_bytes!("../../../../assets/ground/marks.png"), false);
        let detail = load(include_bytes!("../../../../assets/ground/detail.png"), true);
        let normal = load(include_bytes!("../../../../assets/ground/normal.png"), true);
        let image = images.get(&mask).expect("ground mask");
        let resolution = image.width() as usize;
        let pixels = image.data.as_ref().expect("ground mask pixels")
            [..resolution * resolution * 4]
            .to_vec();
        Self {
            mask,
            marks,
            detail,
            normal,
            pixels,
            resolution,
        }
    }
    fn channel(&self, p: Vec2, channel: usize) -> f32 {
        let q = ((p - MASK_ORIGIN) / MASK_SIZE * self.resolution as f32 - Vec2::splat(0.5))
            .clamp(Vec2::ZERO, Vec2::splat(self.resolution as f32 - 1.001));
        let x = q.x as usize;
        let y = q.y as usize;
        let sample = |x, y| self.pixels[(y * self.resolution + x) * 4 + channel] as f32 / 255.;
        let a = sample(x, y) * (1. - q.x.fract()) + sample(x + 1, y) * q.x.fract();
        let b = sample(x, y + 1) * (1. - q.x.fract()) + sample(x + 1, y + 1) * q.x.fract();
        a * (1. - q.y.fract()) + b * q.y.fract()
    }
    /// Matches the reference's interior exclusion and mask-space puddle test.
    pub fn puddle_depth(&self, p: Vec2) -> f32 {
        let interior = p.x > -16. && p.x < 12. && p.y > -28. && p.y < -11.;
        let uv = (p - MASK_ORIGIN) / MASK_SIZE;
        if p.x > 30. || uv.min_element() <= 0. || uv.max_element() >= 1. || interior {
            return -0.2;
        }
        0.5 - self.channel(p, 0)
    }
    pub fn is_puddle(&self, p: Vec2) -> bool {
        self.puddle_depth(p) > 0.015
    }
    pub fn wetness_at(&self, p: Vec2) -> f32 {
        if p.x > 30. {
            return 1.;
        }
        if p.x > -16. && p.x < 12. && p.y > -28. && p.y < -11. {
            return 0.12;
        }
        let uv = (p - MASK_ORIGIN) / MASK_SIZE;
        if uv.min_element() <= 0. || uv.max_element() >= 1. {
            return 0.6;
        }
        let smooth = |a: f32, b: f32, v: f32| {
            let t = ((v - a) / (b - a)).clamp(0., 1.);
            t * t * (3. - 2. * t)
        };
        let depth = self.puddle_depth(p);
        let soak = smooth(-0.11, 0., depth);
        let fringe = smooth(-0.65, -0.02, depth);
        let dry = 1. - smooth(0.2, 0.42, self.channel(p, 1) + fringe * 0.55);
        if depth > 0.015 {
            1.
        } else {
            (0.25 + (1. - dry) * 0.4 + soak * 0.3).min(0.95)
        }
    }
}

/// Reference gravity-driven swell; returns vertical height above mean sea level.
pub fn sea_offset(p: Vec2, time: f32) -> Vec3 {
    let distance = p.distance(Vec2::new(30., 0.));
    let t = ((distance - 50.) / 170.).clamp(0., 1.);
    let fade = 1. - t * t * (3. - 2. * t);
    [
        (-0.92, 0.39, 9.5, 0.06),
        (-0.75, -0.66, 5.7, 0.04),
        (-0.99, 0.12, 3.3, 0.026),
        (-0.5, 0.87, 2., 0.016),
    ]
    .into_iter()
    .map(|(x, z, length, amplitude)| {
        let k = std::f32::consts::TAU / length;
        let direction = Vec2::new(x, z).normalize();
        let phase = k * direction.dot(p) - (9.81 * k).sqrt() * time;
        let horizontal = direction * (amplitude * 0.6 * phase.cos());
        Vec3::new(horizontal.x, amplitude * phase.sin(), horizontal.y)
    })
    .sum::<Vec3>()
        * fade
}
pub fn sea_displacement(p: Vec2, time: f32) -> f32 {
    sea_offset(p, time).y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reference_maps_have_filtered_data_and_world_aligned_puddles() {
        let mut images = Assets::<Image>::default();
        let maps = GroundMaps::new(&mut images);
        assert!(maps.is_puddle(Vec2::new(-30.5, 6.6)));
        assert!(maps.is_puddle(Vec2::new(-19.5, 5.2)));
        assert!(!maps.is_puddle(Vec2::new(-2., -19.5)));
        assert!(!maps.is_puddle(Vec2::new(40., 0.)));
        assert!(!maps.is_puddle(Vec2::new(-100., 0.)));
        assert_eq!(maps.wetness_at(Vec2::new(-2., -19.5)), 0.12);
        assert_eq!(maps.wetness_at(Vec2::new(40., 0.)), 1.);
        for map in [&maps.mask, &maps.marks, &maps.detail, &maps.normal] {
            let image = images.get(map).unwrap();
            assert!(!image.texture_descriptor.format.is_srgb());
            assert!(image.texture_descriptor.mip_level_count > 1);
        }
    }
    #[test]
    fn sea_swell_is_deterministic_bounded_and_fades_at_the_horizon() {
        let p = Vec2::new(30., 0.);
        let start = sea_displacement(p, 0.);
        assert_eq!(start, sea_displacement(p, 0.));
        assert_ne!(start, sea_displacement(p, 1.));
        for t in 0..100 {
            assert!(sea_displacement(p, t as f32 * 0.1).abs() <= 0.1421);
        }
        assert_eq!(sea_displacement(Vec2::new(400., 0.), 1.), 0.);
    }
}

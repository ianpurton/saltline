use bevy::prelude::*;
/// Armor, graphite, machined alloy, rubber, safety orange, optical glass.
pub fn create(a: &mut Assets<StandardMaterial>) -> Vec<Handle<StandardMaterial>> {
    [
        (Color::srgb(0.78, 0.77, 0.65), 0.28, 0.42),
        (Color::srgb(0.035, 0.045, 0.05), 0.65, 0.48),
        (Color::srgb(0.46, 0.51, 0.53), 0.92, 0.25),
        (Color::srgb(0.018, 0.022, 0.024), 0., 0.85),
        (Color::srgb(0.85, 0.22, 0.035), 0.25, 0.45),
        (Color::srgb(0.95, 0.27, 0.04), 0.35, 0.15),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, (base_color, metallic, perceptual_roughness))| {
        a.add(StandardMaterial {
            base_color,
            metallic,
            perceptual_roughness,
            emissive: if i == 5 {
                LinearRgba::rgb(2., 0.35, 0.03)
            } else {
                LinearRgba::BLACK
            },
            ..default()
        })
    })
    .collect()
}

use super::color;
use super::image;


/// Example:
/// fn f(x: f32, y: f32) -> color::Color {
///     color::Color::new(x, y, 0.0, 1.0)
/// }
/// stuff::texture(&mut image, f);
pub fn texture(image: &mut image::Image, f: fn(f32, f32) -> color::Color) {
    for row in 0..image.height() {
        for col in 0..image.width() {
            let x = (col as f32) / (image.width() as f32);
            let y = (row as f32) / (image.height() as f32);
            image.set(col, row, &f(x, y));
        }
    }
}

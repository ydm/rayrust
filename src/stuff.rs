use super::color;
use super::image;


pub fn texture(image: &mut image::Image, f: fn(f32, f32) -> color::Color) {
    for row in 0..image.height() {
        for col in 0..image.width() {
            let x = (row as f32) / (image.height() as f32);
            let y = (col as f32) / (image.width() as f32);
            image.set(row, col, &f(x, y));
        }
    }
}

/*
fn lerp(a: f32, b: f32, u: f32) {
    let v = 1.0 - w;
    a*v + b*w
}

fn dot_grid_gradient(ix: usize, iy: usize, x: f32, y: f32) {
    
}

 // Computes the dot product of the distance and gradient vectors.
 function dotGridGradient(int ix, int iy, float x, float y) {
 
     // Precomputed (or otherwise) gradient vectors at each grid node
     extern float Gradient[IYMAX][IXMAX][2];
 
     // Compute the distance vector
     float dx = x - (float)ix;
     float dy = y - (float)iy;
 
     // Compute the dot-product
     return (dx*Gradient[iy][ix][0] + dy*Gradient[iy][ix][1]);
 }
 
 // Compute Perlin noise at coordinates x, y
 function perlin(float x, float y) {
 
     // Determine grid cell coordinates
     int x0 = (x > 0.0 ? (int)x : (int)x - 1);
     int x1 = x0 + 1;
     int y0 = (y > 0.0 ? (int)y : (int)y - 1);
     int y1 = y0 + 1;
 
     // Determine interpolation weights
     // Could also use higher order polynomial/s-curve here
     float sx = x - (float)x0;
     float sy = y - (float)y0;
 
     // Interpolate between grid point gradients
     float n0, n1, ix0, ix1, value;
     n0 = dotGridGradient(x0, y0, x, y);
     n1 = dotGridGradient(x1, y0, x, y);
     ix0 = lerp(n0, n1, sx);
     n0 = dotGridGradient(x0, y1, x, y);
     n1 = dotGridGradient(x1, y1, x, y);
     ix1 = lerp(n0, n1, sx);
     value = lerp(ix0, ix1, sy);
 
     return value;
 }
*/

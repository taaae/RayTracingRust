use crate::vec3::*;

pub mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    let b = 63;
    for r in 0..=(image_width - 1) {
        eprintln!("lines remaining: {}", image_width - r);
        for g in 0..=(image_height - 1) {
            let pixel_color = Color::new(f64::from(r) / f64::from(image_width - 1),
        f64::from(g) / f64::from(image_height - 1), 0.25);
            write_color(pixel_color);
            
        }
    }
    eprintln!("done!");
}

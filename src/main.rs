fn main() {
    let image_width = 256;
    let image_height = 256;
    println!("P3\n{} {}\n255", image_width, image_height);
    let b = 63;
    for r in 0..=(image_width - 1) {
        eprintln!("lines remaining: {}", image_width - r);
        for g in 0..=(image_height - 1) {
            println!("{} {} {}", r, g, b);
        }
    }
    eprintln!("done!");
}

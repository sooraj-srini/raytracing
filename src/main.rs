use geometric::{vec3, Vector3};

fn write_color(a:&Vector3<f64>) {
    let color = 255.999;
    println!("{} {} {}", (color*a.x()) as i32, (color*a.y()) as i32, (color*a.z()) as i32);
}

fn main() {
    let image_height = 256;
    let image_width = 256;
    eprint!("hi");
    println!("P3\n{image_width} {image_height}\n255");
    for i in 0..image_height{
        eprint!("\rScanning line {i}");
        for j in 0..image_width {
            let rgb = vec3(j as f64/(image_height-1) as f64, i as f64/(image_width-1) as f64, 0.25);

            write_color(&rgb);
        }
    }
}

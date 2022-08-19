pub mod ray;
use geometric::{vec3, Vector3};
use ray::ray::Ray;

fn ray_color(ray:&Ray) -> Vector3<f64> {
    let u = ray.direction().norm();
    let t = 0.5*(u.y() + 1.0);
    return vec3(1.0, 1.0, 1.0)*(1.0 - t) + vec3(0.5, 0.7, 1.0)*t;
}


fn write_color(a:&Vector3<f64>) {
    let color = 255.999;
    println!("{} {} {}", (color*a.x()) as i32, (color*a.y()) as i32, (color*a.z()) as i32);
}

fn main() {
    //Image
    let image_aspect = 16.0/9.0;    
    let image_width = 400;
    let image_height = (image_width as f64/image_aspect) as i32;
   
    //Camera

    let viewport_height = 2.0;
    let viewport_width = image_aspect*viewport_height;
    let focal_length = 1.0;

    let origin = vec3(0.0, 0.0, 0.0);
    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, viewport_height, 0.0);
    let lower_left = origin - horizontal/2.0 - vertical/2.0 - vec3(0.0, 0.0, focal_length);

    eprint!("hi");
    println!("P3\n{image_width} {image_height}\n255");
    for i in (0..image_height).rev() {
        eprint!("\rScanning line {}", image_height-i);
        for j in 0..image_width {
            let u = (j as f64) / (image_width as f64 - 1.0);
            let v = (i as f64) / (image_height as f64 - 1.0);
            let ray = Ray{origin: origin, dir: lower_left + horizontal*u + vertical*v - origin};
            let color = ray_color(&ray);
            write_color(&color);
        }
    }
}

fn main() {
    let image_height = 256;
    let image_width = 256;
    println!("P3\n{image_width} {image_height}\n255");
    for i in 0..image_height{
        for j in 0..image_width {
            let r:f64 = j as f64/(image_height-1) as f64;
            let g:f64 = i as f64/(image_width-1) as f64;
            let b = 0.25;
            let ir = (r*255.999) as i32;
            let ig = (g*255.999) as i32;
            let ib = (b*255.999) as i32;
            println!("{ir} {ig} {ib}");
        }
    }
}

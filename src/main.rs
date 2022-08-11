mod vec3;
mod ray;

fn main() {
    let w = 256;
    let h = 256;
    let max_value = 255;

    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev() {
        for i in 0..w {

            let r = i as f32 / w as f32;
            let g = j as f32 / h as f32;
            let b = 1 as f32;

            let ir = (255.99 * r) as u32;
            let ig = (255.99 * g) as u32;
            let ib = (255.99 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

mod vec3;
mod ray;

use ray::Ray;
use crate::vec3::Vec3;

fn ray_clr(r: &Ray) -> Vec3 {
    let unit_direction = Vec3::normalize(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn write_clr(px_clr: Vec3) {
    let r = (255.99 * px_clr.r()) as u32;
    let g = (255.99 * px_clr.g()) as u32;
    let b = (255.99 * px_clr.b()) as u32;

    println!("{} {} {}", r, g, b);
}


fn main() {
    let w = 256;
    let h = 256;
    let max_value = 255;


    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);

    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev() {
        for i in 0..w {

            let u = i as f32 / w as f32;
            let v = j as f32 / h as f32;

            let r= Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            write_clr(ray_clr(&r));
        }
    }
}

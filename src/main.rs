mod vec3;

use vec3::Vec3;

fn main() {
    let u = Vec3::new(1.0, 2.0, 0.0);

    println!("{}", Vec3::normalize(&u));
    println!("{}", u);
}

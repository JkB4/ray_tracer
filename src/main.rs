mod vec3;

use vec3::Vec3;

fn main() {
    let v = Vec3::new(1.0, 2.0, 0.0);
    let f = v.length();
    let u = Vec3::new(2.0, 3.0, 1.0);
    println!("{}", v + u);
}

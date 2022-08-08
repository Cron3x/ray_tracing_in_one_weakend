mod image_fromats;
mod vec3_mod;
mod ray;

use ray::Ray;
use vec3_mod::{Vec3, unit_vector};

fn ray_color(r:Ray){
    let unit_direction: Vec3 = unit_vector(r.direction());
    //TODO
}

fn main() {


    //image_fromats::ppm::PPMImageFormat::new(10,10).save("./t.ppm".to_string()).unwrap();
    image_fromats::ppm::PPMImageFormat::new(256,256).save("./t.ppm".to_string()).unwrap();
}

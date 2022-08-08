mod image_fromats;
mod vec3;
mod ray;

fn main() {
    //image_fromats::ppm::PPMImageFormat::new(10,10).save("./t.ppm".to_string()).unwrap();
    image_fromats::ppm::PPMImageFormat::new(256,256).save("./t.ppm".to_string()).unwrap();
}

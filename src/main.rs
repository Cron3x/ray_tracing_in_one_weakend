mod image_fromats;
mod vec3;
mod ray;
mod color;
mod hittable;
mod sphere;

use hittable::Hittable;
use image_fromats::ppm::PPMImageFormat;
use ray::Ray;
use sphere::Sphere;
use vec3::{Vec3, unit_vector, vec3, Color, Point3, rgb};

fn ray_color(r:&Ray) -> Color{
    let t = Sphere::new(vec3(0.0,0.0,-1.0), 0.5).hit(r, 0, 10, \/);
    if t > 0.0{
        let n:Vec3 = unit_vector(r.at(t) - vec3(0.0,0.0,-1.0));
        return rgb(n.x+1f64,n.y+1f64,n.z+1f64)*0.5;
    }
    //println!("{:?}",r.direction());
    let unit_direction: Vec3 = unit_vector(r.direction());
    let t:f64 = 0.5 * (unit_direction.y +1.0);
    rgb(1.0, 1.0, 1.0)*(1.0-t) + rgb(0.5, 0.7, 1.0)*t
}
fn main() {
    // Image
    let aspect_ration = 16.0 / 9.0;
    let image_width = 400;
    let image_height:i32 = (image_width as f64 / aspect_ration) as i32; 
    
    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ration * viewport_height;
    let focal_length = 1.0;
    
    let origin:Point3 = vec3(0f64,0f64,0f64);
    let horizontal:Vec3 = vec3(viewport_width, 0f64, 0f64);
    let vertical:Vec3 = vec3(0f64, viewport_height, 0f64);
    let lower_left_corner = origin - horizontal/2f64 - vertical/2f64 - vec3(0f64, 0f64, focal_length);
    // Render

    //image_fromats::ppm::PPMImageFormat::new(10,10).save("./t.ppm".to_string()).unwrap();
    
    let mut image_vec:Vec<String> = Vec::new();

    image_vec.push("P3".to_string());
    image_vec.push(format!("{image_width} {image_height}"));
    image_vec.push("255".to_string());

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            
            let u:f64 = i as f64 / (image_width-1) as f64;
            let v:f64 = j as f64 / (image_height-1) as f64;
            let r:Ray = Ray{orig: origin, dir: lower_left_corner + horizontal*u + vertical*v - origin};
            let pixel_color = ray_color(&r);
            let px = color::write_color(pixel_color);
            image_vec.push(px);
        }
    }
    let img_file:PPMImageFormat = PPMImageFormat { image_vec: image_vec };
    img_file.save("./dbg.ppm".to_string()).unwrap();
    println!("\nDone.");
}

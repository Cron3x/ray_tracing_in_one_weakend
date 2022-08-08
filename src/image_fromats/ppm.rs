use std::{fs::File, io::Write};

use crate::vec3::{color, self, Color, Vec3};


pub struct PPMImageFormat {
	pub image_vec: Vec<String>,
}
impl PPMImageFormat {
	pub fn new(width: usize, height: usize) -> Self {
		let mut image_vec:Vec<String> = Vec::new();

		image_vec.push("P3".to_string());
		image_vec.push(format!("{width} {height}"));
		image_vec.push("255".to_string());

		for j in 0..height+2 {
			print!("\rScalines remaining: {}", (height+2) - j);
			if j < 2 {
				continue;
			}
			for i in 0..width {
				let pixel_color:Color = Vec3(i as f64 / (width as f64 - 1 as f64), j as f64 / (height as f64 - 1 as f64), 0.25);

				image_vec.push(color::write_color(pixel_color));
			}
		}
		print!("\nDone");
		Self {
			image_vec: image_vec,
		}
	}
	pub fn save(&self, location:String) -> std::io::Result<()>{
		let mut file = File::create(location)?;
		for i in &self.image_vec {
			writeln!(file, "{i}")?;
		}
		Ok(())
	}
	pub fn dbg_draw(&mut self) -> &mut Self{
		println!("{:?}", self.image_vec);
		for i in &self.image_vec {
				dbg!(i);
		}
		self
	}
}
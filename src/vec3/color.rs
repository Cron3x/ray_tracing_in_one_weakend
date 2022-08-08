use super::Color;

pub fn write_color(color:Color) -> String{
	let ir:i32 = (255.999 * color.0) as i32;
	let ig:i32 = (255.999 * color.1) as i32;
	let ib:i32 = (255.999 * color.2) as i32;
	format!("{ir} {ig} {ib} ")
}
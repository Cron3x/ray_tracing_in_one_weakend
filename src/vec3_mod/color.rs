use super::Color;

pub fn write_color(color:Color) -> String{
	let ir:i32 = (255.999 * color.x) as i32;
	let ig:i32 = (255.999 * color.y) as i32;
	let ib:i32 = (255.999 * color.z) as i32;
	format!("{ir} {ig} {ib} ")
}
use mlx::*;

use crate::Game;

pub fn copy_pixel(dst: &Image, dst_x: u32, dst_y: u32) {	unsafe {
		let mut start = dst.data().add((dst_y * dst.line_size() + dst_x * dst.bytes_per_pixel()) as usize) ;
		for _ in 0..dst.bytes_per_pixel() {
			*start = 255;
			start=start.add(1);
		}
	}
}

pub fn draw_square(size: u32, x: u32, y:u32, weight: u32, img: &Image) {
	for j in 0..weight {
		for i in 0..size - j * 2 {
			copy_pixel(img, j + x + i, j + y);
			copy_pixel(img, j + x, j + y + i);
			copy_pixel(img, j + x + i, y + size - j);
			copy_pixel(img, x + size - j, j + y + i);
		}
	}
}

pub fn draw(game: &Game, output: &Image) {
	
}
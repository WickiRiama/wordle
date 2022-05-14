use mlx::*;

use crate::*;

pub fn set_pixel(dst: &Image, dst_x: u32, dst_y: u32, grayscale: u8) {
	assert!(dst_x < dst.width(), "invalid dst X value ({})", dst_x);
	assert!(dst_y < dst.height(), "invalid dst Y value ({})", dst_y);
	
	unsafe {
		let mut start = dst.data().add((dst_y * dst.line_size() + dst_x * dst.bytes_per_pixel()) as usize) ;
		for _ in 0..dst.bytes_per_pixel() {
			*start = grayscale;
			start=start.add(1);
		}
	}
}

pub fn copy_pixel(dst: &Image, dst_x: u32, dst_y: u32, mut pixel: *const u8) {
	assert!(dst_x < dst.width(), "invalid dst X value ({})", dst_x);
	assert!(dst_y < dst.height(), "invalid dst Y value ({})", dst_y);
	
	unsafe {
		let mut start = dst.data().add((dst_y * dst.line_size() + dst_x * dst.bytes_per_pixel()) as usize) ;
		for _ in 0..dst.bytes_per_pixel() {
			*start = *pixel;
			start = start.add(1);
			pixel = pixel.add(1);
		}
	}
}


pub fn draw_square(size: u32, x: u32, y:u32, weight: u32, img: &Image) {
	for j in 0..weight {
		for i in 0..=size - j * 2 {
			set_pixel(img, j + x + i, j + y, 0);
			set_pixel(img, j + x, j + y + i, 0);
			set_pixel(img, j + x + i, y + size - j, 0);
			set_pixel(img, x + size - j, j + y + i, 0);
		}
	}
}

pub fn draw_n_squares(img: &Image, nb_col: u32, nb_row: u32, size: u32)
{
	let mut x = 30;
	let mut y;
	for _ in 0..nb_col{
		y = 30;
		for __ in 0..nb_row{
			draw_square(size, x, y, 2, img);
			y = y + size + 10;
		}
		x = x + size + 10;
	}
}

pub fn init_bg(img: &Image){
	for i in 0..img.width(){
		for j in 0..img.height(){
			set_pixel(img, i, j, 255);
		}
	}
	draw_n_squares(img, 5, 6, 64);
}

pub fn draw_letter(letter: Letter, dst_x: u32, dst_y: u32, dst_img: &Image, alphabet: &Image){
	let index = letter as u32;
	let x_alphabet = index * 64;
	let y_alphabet = 0 as u32;
	for y in 0..64{
		for x in 0..64{
			let color = unsafe { alphabet.data().add((alphabet.line_size() * (y_alphabet + y) + alphabet.bytes_per_pixel() * (x_alphabet + x)) as usize) };
			copy_pixel(dst_img, dst_x + x, dst_y + y, color);
		}
	}
}

pub fn draw_current(word: [Letter; 5], row: u32, cursor: usize, img: &Image, alphabet: &Image){
	let mut x = 30;
	let y = 30 + row * 64 + row * 10;
	for i in 0..cursor{
		draw_letter(word[i], x, y, img, alphabet);
		x = x + 74;
	}
}

pub fn draw(game: &Game, output: &Image, images: &Images) {
	init_bg(output);
	draw_current(game.current_word, 0, game.cursor, output, &images.black_letters);
}
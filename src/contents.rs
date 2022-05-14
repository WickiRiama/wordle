use mlx::{Image, Mlx};

use crate::cstr;

const IMAGE_LOAD_ERROR: &str = "Failed to load an image";

pub struct Images<'a> {
	pub black_letters: Image<'a>,
	pub green_letters: Image<'a>,
	pub grey_letters: Image<'a>,
	pub yellow_letters: Image<'a>,
}

impl<'a> Images<'a> {
	pub fn load(mlx: &Mlx<'a>) -> Self {
		Self {
			black_letters: mlx.create_image_from_xpm_file(cstr("assets/alphabet_black.xpm\0")).expect(IMAGE_LOAD_ERROR),
			green_letters: mlx.create_image_from_xpm_file(cstr("assets/alphabet_green.xpm\0")).expect(IMAGE_LOAD_ERROR),
			yellow_letters: mlx.create_image_from_xpm_file(cstr("assets/alphabet_yellow.xpm\0")).expect(IMAGE_LOAD_ERROR),
			grey_letters: mlx.create_image_from_xpm_file(cstr("assets/alphabet_grey.xpm\0")).expect(IMAGE_LOAD_ERROR),
		}
	}
}
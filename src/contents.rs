use mlx::{Image, Mlx};

use crate::cstr;

const IMAGE_LOAD_ERROR: &str = "Failed to load an image";

pub struct Images {
    pub black_letters: Image,
    pub green_letters: Image,
    pub grey_letters: Image,
    pub yellow_letters: Image,
    pub black_letters_32: Image,
    pub green_letters_32: Image,
    pub grey_letters_32: Image,
    pub yellow_letters_32: Image,}

impl Images {
    pub fn load(mlx: &Mlx) -> Self {
        Self {
            black_letters: mlx
                .create_image_from_xpm_file(cstr("assets/alphabet_black.xpm\0"))
                .unwrap_or_else(|_| panic!("{}", IMAGE_LOAD_ERROR)),
            green_letters: mlx
                .create_image_from_xpm_file(cstr("assets/alphabet_green.xpm\0"))
                .unwrap_or_else(|_| panic!("{}", IMAGE_LOAD_ERROR)),
            yellow_letters: mlx
                .create_image_from_xpm_file(cstr("assets/alphabet_yellow.xpm\0"))
                .unwrap_or_else(|_| panic!("{}", IMAGE_LOAD_ERROR)),
            grey_letters: mlx
                .create_image_from_xpm_file(cstr("assets/alphabet_grey.xpm\0"))
                .unwrap_or_else(|_| panic!("{}", IMAGE_LOAD_ERROR)),
			black_letters_32: mlx
                .create_image_from_xpm_file(cstr("assets/alphabet_black_grey_border_32.xpm\0"))
                .unwrap_or_else(|_| panic!("{}", IMAGE_LOAD_ERROR)),
            green_letters_32: mlx
                .create_image_from_xpm_file(cstr("assets/alphabet_green_grey_border_32.xpm\0"))
                .unwrap_or_else(|_| panic!("{}", IMAGE_LOAD_ERROR)),
            yellow_letters_32: mlx
                .create_image_from_xpm_file(cstr("assets/alphabet_yellow_grey_border_32.xpm\0"))
                .unwrap_or_else(|_| panic!("{}", IMAGE_LOAD_ERROR)),
            grey_letters_32: mlx
                .create_image_from_xpm_file(cstr("assets/alphabet_grey_grey_border_32.xpm\0"))
                .unwrap_or_else(|_| panic!("{}", IMAGE_LOAD_ERROR)),
        }
    }
}

use mlx::{Image, Mlx};

use crate::cstr;

const IMAGE_LOAD_ERROR: &str = "Failed to load an image";

pub struct Images {
    pub black_letters: Image,
    pub green_letters: Image,
    pub grey_letters: Image,
    pub yellow_letters: Image,
}

impl Images {
    /// ## Safety
    /// 
    /// The created instance must be dropped after `mlx`.
    pub unsafe fn load(mlx: &Mlx) -> Self {
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
        }
    }
}

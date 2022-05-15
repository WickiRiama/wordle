use mlx::{Image, Mlx};

use crate::cstr;

const IMAGE_LOAD_ERROR: &str = "Failed to load an image";

pub struct Images {
    pub black_letters: Image,
    pub green_letters: Image,
    pub grey_letters: Image,
    pub yellow_letters: Image,
    pub won_final_screen: Image,
    pub lost_final_screen: Image,
    pub winning_letters: Image,
}

impl Images {
    /// ## Safety
    /// 
    /// The created instance must be dropped after `mlx`.
    pub unsafe fn load(mlx: &Mlx) -> Self {
        unsafe fn load_image(mlx: &Mlx, name: &str) -> Image {
            mlx
                .create_image_from_xpm_file(cstr(name))
                .unwrap_or_else(|_| panic!("{}", IMAGE_LOAD_ERROR))
        }

        Self {
            black_letters: load_image(mlx, "assets/alphabet_black.xpm\0"),
            green_letters: load_image(mlx, "assets/alphabet_green.xpm\0"),
            yellow_letters: load_image(mlx, "assets/alphabet_yellow.xpm\0"),
            grey_letters: load_image(mlx, "assets/alphabet_grey.xpm\0"),
            won_final_screen: load_image(mlx, "assets/alphabet_grey.xpm\0"),
            lost_final_screen: load_image(mlx, "assets/alphabet_grey.xpm\0"),
            winning_letters: load_image(mlx, "assets/alphabet_green_no_border.xpm\0"),
        }
    }
}

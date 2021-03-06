use mlx::{Image, Mlx};

use crate::cstr;

pub struct Images {
    pub black_letters: Image,
    pub green_letters: Image,
    pub grey_letters: Image,
    pub yellow_letters: Image,
    pub won_final_screen: Image,
    pub lost_final_screen: Image,
    pub winning_letters: Image,
    pub black_letters_32: Image,
    pub green_letters_32: Image,
    pub grey_letters_32: Image,
    pub yellow_letters_32: Image,
}

impl Images {
    /// ## Safety
    ///
    /// The created instance must be dropped after `mlx`.
    pub unsafe fn load(mlx: &Mlx) -> Self {
        unsafe fn load_image(mlx: &Mlx, name: &str) -> Image {
            mlx.create_image_from_xpm_file(cstr(name))
                .unwrap_or_else(|_| panic!("Failed to load '{}'.", name))
        }

        Self {
            black_letters: load_image(mlx, "assets/alphabet_neutral.xpm\0"),
            green_letters: load_image(mlx, "assets/alphabet_correct.xpm\0"),
            yellow_letters: load_image(mlx, "assets/alphabet_misplaced.xpm\0"),
            grey_letters: load_image(mlx, "assets/alphabet_incorrect.xpm\0"),
            won_final_screen: load_image(mlx, "assets/you_won.xpm\0"),
            lost_final_screen: load_image(mlx, "assets/you_lost.xpm\0"),
            winning_letters: load_image(mlx, "assets/alphabet_correct.xpm\0"),
            black_letters_32: load_image(mlx, "assets/alphabet_neutral_keyboard.xpm\0"),
            green_letters_32: load_image(mlx, "assets/alphabet_correct_keyboard.xpm\0"),
            yellow_letters_32: load_image(mlx, "assets/alphabet_misplaced_keyboard.xpm\0"),
            grey_letters_32: load_image(mlx, "assets/alphabet_incorrect_keyboard.xpm\0"),
        }
    }
}

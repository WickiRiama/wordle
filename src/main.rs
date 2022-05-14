use mlx::*;
use std::ffi::CStr;

mod display;
use display::*;

mod game;
use game::*;

#[inline]
fn cstr(s: &str) -> &CStr {
    CStr::from_bytes_with_nul(s.as_bytes()).unwrap()
}

const WIDTH: u32 = 420;
const HEIGHT: u32 = 494;

fn main() {
    let mlx = Mlx::init().expect("Failed to initialize the MiniLibX.");

    let win = mlx.create_window(WIDTH, HEIGHT, cstr("Test\0")).expect("Failed to initialize the window.");
    let img = mlx.create_image(WIDTH, HEIGHT).unwrap();
    let black_alpha = mlx.create_image_from_xpm_file(cstr("assets/alphabet_black.xpm\0")).unwrap();
    let green_alpha = mlx.create_image_from_xpm_file( cstr("assets/alphabet_green.xpm\0")).unwrap();
    let yellow_alpha = mlx.create_image_from_xpm_file( cstr("assets/alphabet_yellow.xpm\0")).unwrap();
    let grey_alpha = mlx.create_image_from_xpm_file(cstr("assets/alphabet_grey.xpm\0")).unwrap();
    init_bg(&img);
    win.put_image(&img, 0, 0);
    mlx.hook_loop(move || {});
    win.hook(|KeyPress(keycode)| {
        if keycode == KeyCode::ESCAPE {
            mlx.stop_loop();
        }
    });
    win.hook(|Destroy| {
        mlx.stop_loop();
    });

    mlx.start_loop();
}

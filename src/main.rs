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

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let mlx = Mlx::init().expect("Failed to initialize the MiniLibX.");

    let win = mlx.create_window(WIDTH, HEIGHT, cstr("Test\0")).expect("Failed to initialize the window.");
    let img = mlx.create_image(WIDTH, HEIGHT).unwrap();
    let black_alpha = mlx.create_image_from_xpm_file(cstr("assets/alphabet_black.xpm\0")).unwrap();
    let green_alpha = mlx.create_image_from_xpm_file( cstr("assets/alphabet_green.xpm\0")).unwrap();
    let yellow_alpha = mlx.create_image_from_xpm_file( cstr("assets/alphabet_yellow.xpm\0")).unwrap();
    let grey_alpha = mlx.create_image_from_xpm_file(cstr("assets/alphabet_grey.xpm\0")).unwrap();
    draw_square(64, 640, 360, 4, &img);
    win.put_image(&img, 0, 0);
    win.put_image(&black_alpha, 0, 0);
    win.put_image(&yellow_alpha, 0, 64);
    win.put_image(&green_alpha, 0, 128);
    win.put_image(&grey_alpha, 0, 192);
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

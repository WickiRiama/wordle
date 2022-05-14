use mlx::*;
use std::cell::RefCell;
use std::ffi::CStr;
use std::rc::Rc;

mod display;
use display::*;

mod game;
use game::*;

mod custom_panic;

mod contents;
use contents::*;

#[inline]
fn cstr(s: &str) -> &CStr {
    CStr::from_bytes_with_nul(s.as_bytes()).unwrap()
}

const WIDTH: u32 = 420;
const HEIGHT: u32 = 494;

fn main() {
    // custom_panic::set_custom_panic_hook();

    let game = Rc::new(RefCell::new(Game::new([Letter::A, Letter::B, Letter::C, Letter::D, Letter::E], Vec::new())));

    
    let mlx = Mlx::init().expect("Failed to initialize the MiniLibX.");
    mlx.set_autorepeat(false);
    let images = Images::load(&mlx);
    let win = mlx.create_window(WIDTH, HEIGHT, cstr("Test\0")).expect("Failed to initialize the window.");
    let img = mlx.create_image(WIDTH, HEIGHT).unwrap();
    
    let win2 = win.clone();
    let game2 = game.clone();
    mlx.hook_loop(move || {
        draw(&game2.borrow(), &img, &images);
        win2.put_image(&img, 0, 0);
    });

    let mlx2 = mlx.clone();
    win.hook(move |KeyPress(keycode)| {
        let mut game = game.borrow_mut();

        if keycode == KeyCode::ESCAPE {
            mlx2.stop_loop();
            return;
        }

        match keycode {
            KeyCode::A => game.type_letter(Letter::A),
            KeyCode::B => game.type_letter(Letter::B),
            KeyCode::C => game.type_letter(Letter::C),
            KeyCode::D => game.type_letter(Letter::D),
            KeyCode::E => game.type_letter(Letter::E),
            KeyCode::F => game.type_letter(Letter::F),
            KeyCode::G => game.type_letter(Letter::G),
            KeyCode::H => game.type_letter(Letter::H),
            KeyCode::I => game.type_letter(Letter::I),
            KeyCode::J => game.type_letter(Letter::J),
            KeyCode::K => game.type_letter(Letter::K),
            KeyCode::L => game.type_letter(Letter::L),
            KeyCode::M => game.type_letter(Letter::M),
            KeyCode::N => game.type_letter(Letter::N),
            KeyCode::O => game.type_letter(Letter::O),
            KeyCode::P => game.type_letter(Letter::P),
            KeyCode::Q => game.type_letter(Letter::Q),
            KeyCode::R => game.type_letter(Letter::R),
            KeyCode::S => game.type_letter(Letter::S),
            KeyCode::T => game.type_letter(Letter::T),
            KeyCode::U => game.type_letter(Letter::U),
            KeyCode::V => game.type_letter(Letter::V),
            KeyCode::W => game.type_letter(Letter::W),
            KeyCode::X => game.type_letter(Letter::X),
            KeyCode::Y => game.type_letter(Letter::Y),
            KeyCode::Z => game.type_letter(Letter::Z),
            KeyCode::BACKSPACE => game.cancel_letter(),
            KeyCode::RETURN => game.confirm_word(),
            _ => (),
        }
    });

    let mlx3 = mlx.clone();
    win.hook(move |Destroy| {
        mlx3.stop_loop();
    });

    mlx.start_loop();
}

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

fn create_dict() -> Vec<[Letter; 5]> {
    let mut vec = Vec::<u8>::new();
    
    unsafe {
        let mut count: libc::ssize_t;

        let fd = libc::open(b"words.txt\0".as_ptr() as _, libc::O_RDONLY);
        
        if fd < 0 {
            panic!("Failed to open 'words.txt'.");
        }
        
        loop {
            vec.reserve(2048);
            count = libc::read(fd, vec.as_mut_ptr().add(vec.len()) as _, vec.capacity() - vec.len());

            if count < 0 {
                panic!("Failed to read from 'words.txt'");
            }

            if count == 0 {
                break;
            }

            vec.set_len(vec.len() + count as usize);
        }
    };

    vec
        .split(|c| *c == b'\n')
        .enumerate()
        .map(|(i, s)| {
            match *s {
                [a, b, c, d, e] => match (
                    Letter::from_ascii_char(a),
                    Letter::from_ascii_char(b),
                    Letter::from_ascii_char(c),
                    Letter::from_ascii_char(d),
                    Letter::from_ascii_char(e),
                ) {
                    (Some(a), Some(b), Some(c), Some(d), Some(e)) => return [a, b, c, d, e],
                    _ => {}
                },
                _ => {}
            }

            panic!("Wrong word on line {}: '{}'", i + 1, s.escape_ascii());
        })
        .collect()
}

fn main() {
    // Installs a custom panic hook so that error messages are properly displayed on
    // error.
    custom_panic::set_custom_panic_hook();

    // Safety:
    //  `srand` has no safety caveats and can be called with any seed value.
    //  `time` can be called with `NULL`.
    unsafe { libc::srand(libc::time(std::ptr::null_mut()) as u32) };

    let game = Rc::new(RefCell::new(Game::new(create_dict())));

    // Initialize MiniLibX and load the images.
    let mlx = Mlx::init().unwrap_or_else(|_| panic!("Failed to initialize the MiniLibX."));

    // This image is used to draw on the whole screen.
    let win = unsafe { mlx
        .create_window(WIDTH, HEIGHT, cstr("Wordle\0")) }
        .unwrap_or_else(|_| panic!("Failed to create a window."));

    let _h = unsafe { win.hook(|Destroy| {
        win.mlx().stop_loop();
    }) };

    let _h = unsafe {
        win.hook(|KeyPress(keycode)| {
            let mut game = game.borrow_mut();
    
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
                KeyCode::ESCAPE => win.mlx().stop_loop(),
                _ => (),
            }
        })
    };

    let canvas = unsafe { win.mlx().create_image(WIDTH, HEIGHT).unwrap() } ;
    let images = unsafe { Images::load(win.mlx()) };
    mlx.start_loop(|| {
        draw(&game.borrow(), &canvas, &images);
        win.put_image(&canvas, 0, 0);
    });
}

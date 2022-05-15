use mlx::*;

use crate::*;

use Letter::*;

const LAYOUT: [Letter; 26] = [
    Q, W, E, R, T, Y, U, I, O, P, A, S, D, F, G, H, J, K, L, Z, X, C, V, B, N, M,
];

fn set_pixel(dst: &Image, dst_x: u32, dst_y: u32, grayscale: u8) {
    assert!(dst_x < dst.width(), "invalid dst X value ({})", dst_x);
    assert!(dst_y < dst.height(), "invalid dst Y value ({})", dst_y);

    unsafe {
        let mut start = dst
            .data()
            .add((dst_y * dst.line_size() + dst_x * dst.bytes_per_pixel()) as usize);
        for _ in 0..dst.bytes_per_pixel() {
            *start = grayscale;
            start = start.add(1);
        }
    }
}

fn copy_pixel(dst: &Image, dst_x: u32, dst_y: u32, mut pixel: *const u8) {
    assert!(dst_x < dst.width(), "invalid dst X value ({})", dst_x);
    assert!(dst_y < dst.height(), "invalid dst Y value ({})", dst_y);

    unsafe {
        let mut start = dst
            .data()
            .add((dst_y * dst.line_size() + dst_x * dst.bytes_per_pixel()) as usize);
        for _ in 0..dst.bytes_per_pixel() {
            *start = *pixel;
            start = start.add(1);
            pixel = pixel.add(1);
        }
    }
}

fn draw_square(size: u32, x: u32, y: u32, weight: u32, img: &Image) {
    for j in 0..weight {
        for i in 0..=size - 1 - j * 2 {
            set_pixel(img, x + j + i, y + j, 0xca);
            set_pixel(img, x + j, y + i - j, 0xca);
            set_pixel(img, x + j + i, y - j + size - 1, 0xca);
            set_pixel(img, x - j + size - 1, y + j + i, 0xca);
        }
    }
}

fn draw_n_squares(img: &Image, nb_col: u32, nb_row: u32, size: u32) {
    let mut x = 55;
    let mut y;
    for _ in 0..nb_col {
        y = 30;
        for __ in 0..nb_row {
            draw_square(size, x, y, 2, img);
            y = y + size + 10;
        }
        x = x + size + 10;
    }
}

fn init_bg(img: &Image) {
    for i in 0..img.width() {
        for j in 0..img.height() {
            set_pixel(img, i, j, 255);
        }
    }
    draw_n_squares(img, 5, 6, 64);
}

fn draw_letter(
    letter: Letter,
    dst_x: u32,
    dst_y: u32,
    dst_img: &Image,
    alphabet: &Image,
    size: u32,
) {
    let index = letter as u32;
    let x_alphabet = index * size;
    let y_alphabet = 0u32;
    for y in 0..size {
        for x in 0..size {
            let color = unsafe {
                alphabet.data().add(
                    (alphabet.line_size() * (y_alphabet + y)
                        + alphabet.bytes_per_pixel() * (x_alphabet + x))
                        as usize,
                )
            };
            copy_pixel(dst_img, dst_x + x, dst_y + y, color);
        }
    }
}

fn draw_current(word: [Letter; 5], row: u32, cursor: usize, img: &Image, alphabet: &Image) {
    let mut x = 55;
    let y = 30 + row * 64 + row * 10;
    for letter in &word[0..cursor] {
        draw_letter(*letter, x, y, img, alphabet, 64);
        x += 74;
    }
}

fn draw_previous(word: [(Letter, Correctness); 5], row: u32, img: &Image, images: &Images) {
    let mut x = 55;
    let y = 30 + row * 64 + row * 10;
    let mut alphabet: &Image;
    for (letter, correctness) in word {
        match correctness {
            Correctness::Correct => alphabet = &images.green_letters,
            Correctness::Misplaced => alphabet = &images.yellow_letters,
            Correctness::Incorrect => alphabet = &images.grey_letters,
        }
        draw_letter(letter, x, y, img, alphabet, 64);
        x += 74;
    }
}

fn copy_image(source: &Image, dst_x: u32, dst_y: u32, destination: &Image) {
    for y in 0..source.height() {
        for x in 0..source.width() {
            copy_pixel(destination, dst_x + x, dst_y + y, unsafe {
                source.data().add((x * source.bytes_per_pixel() + source.line_size() * y) as usize)
            });
        }
    }
}

fn draw_final_screen(
    word: [Letter; Game::WORD_SIZE],
    target: &Image,
    image: &Image,
    alphabet: &Image,
) {
    copy_image(image, 10, 494, target);

    for (i, &letter) in word.iter().enumerate() {
        draw_letter(letter, 75 + 64 * i as u32, 540, target, alphabet, 64);
    }
}

fn draw_keyboard(img: &Image, images: &Images, game: &Game) {
    let mut alphabet: &Image;
    let mut x = 30u32;
    let mut y = 494;

    for &letter in &LAYOUT[0..10] {
        match game.letters_state[letter as usize] {
            Some(Correctness::Correct) => alphabet = &images.green_letters_32,
            Some(Correctness::Misplaced) => alphabet = &images.yellow_letters_32,
            Some(Correctness::Incorrect) => alphabet = &images.grey_letters_32,
            None => alphabet = &images.black_letters_32,
        }
        draw_letter(letter, x, y, img, alphabet, 32);
        x += 42;
    }
    y += 42;
    x = 51;
    for &letter in &LAYOUT[10..19] {
        match game.letters_state[letter as usize] {
            Some(Correctness::Correct) => alphabet = &images.green_letters_32,
            Some(Correctness::Misplaced) => alphabet = &images.yellow_letters_32,
            Some(Correctness::Incorrect) => alphabet = &images.grey_letters_32,
            None => alphabet = &images.black_letters_32,
        }
        draw_letter(letter, x, y, img, alphabet, 32);
        x += 42;
    }
    y += 42;
    x = 93;
    for &letter in &LAYOUT[19..26] {
        match game.letters_state[letter as usize] {
            Some(Correctness::Correct) => alphabet = &images.green_letters_32,
            Some(Correctness::Misplaced) => alphabet = &images.yellow_letters_32,
            Some(Correctness::Incorrect) => alphabet = &images.grey_letters_32,
            None => alphabet = &images.black_letters_32,
        }
        draw_letter(letter, x, y, img, alphabet, 32);
        x += 42;
    }
}

pub fn draw(game: &Game, output: &Image, images: &Images) {
    init_bg(output);
    for i in 0..game.current_try {
        draw_previous(game.previous_words[i], i as u32, output, images);
    }
    draw_current(
        game.current_word,
        game.current_try as u32,
        game.cursor,
        output,
        &images.black_letters,
    );

    match game.state {
        GameState::Playing => draw_keyboard(output, images, game),
        GameState::Lost => draw_final_screen(
            game.winning_word,
            output,
            &images.lost_final_screen,
            &images.winning_letters,
        ),
        GameState::Won => draw_final_screen(
            game.winning_word,
            output,
            &images.won_final_screen,
            &images.winning_letters,
        ),
    }
}

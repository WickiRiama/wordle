//! This module exposes the functions and types defined in [`mlx.h`].
//! 
//! [`mlx.h`]: https://github.com/42Paris/minilibx-linux/blob/master/mlx.h

use std::{ffi::c_void, os::raw::{c_int, c_char}};

/// The type returned by [`init`] and passed to every other function of MLX.
pub type Mlx = *mut c_void;

/// The type returned by [`new_window`].
pub type Window = *mut c_void;

/// The type returned by [`new_image`].
pub type Image = *mut c_void;

extern "C" {
	pub fn mlx_init() -> Mlx;
	pub fn mlx_new_window(mlx: Mlx, width: c_int, height: c_int, name: *const c_char) -> Window;
	pub fn mlx_clear_window(mlx: Mlx, win: Window) -> c_int;
	pub fn mlx_pixel_put(mlx: Mlx, win: Window, x: c_int, y: c_int, color: c_int) -> c_int;
	pub fn mlx_new_image(mlx: Mlx, width: c_int, height: c_int) -> Image;
	pub fn mlx_get_data_addr(mlx: Mlx, bits_per_pixel: *mut c_int, size_line: *mut c_int, endian: *mut c_int) -> *mut c_char;
	pub fn mlx_put_image_to_window(mlx: Mlx, win: Window, img: Image, x: c_int, y: c_int) -> c_int;
	pub fn mlx_get_color_value(mlx: Mlx, color: c_int) -> c_int;
	pub fn mlx_loop_hook(mlx: Mlx, funct_ptr: unsafe extern "C" fn(param: *mut c_void) -> c_int, param: *mut c_void) -> c_int;
	pub fn mlx_loop(mlx: Mlx) -> c_int;
	pub fn mlx_loop_end(mlx: Mlx) -> c_int;
	pub fn mlx_string_put(mlx: Mlx, win: Window, x: c_int, y: c_int, color: c_int, string: *const c_char) -> c_int;
	pub fn mlx_set_font(mlx: Mlx, win: Window, name: *const c_char);
	pub fn mlx_xpm_to_image(mlx: Mlx, win: Window, name: *const c_char) -> Image;
	pub fn mlx_xpm_file_to_image(mlx: Mlx, filename: *const c_char, width: *mut c_int, height: *mut c_int) -> Image;
	pub fn mlx_destroy_window(mlx: Mlx, win: Window) -> c_int;
	pub fn mlx_destroy_image(mlx: Mlx, img: Image) -> c_int;
	pub fn mlx_destroy_display(mlx: Mlx) -> c_int;
	pub fn mlx_hook(win: Window, x_event: c_int, x_mask: c_int, funct: usize, param: *mut c_void) -> c_int;
	pub fn mlx_do_key_autorepeatoff(mlx: Mlx) -> c_int;
	pub fn mlx_do_key_autorepeaton(mlx: Mlx) -> c_int;
	pub fn mlx_do_sync(mlx: Mlx) -> c_int;
	pub fn mlx_mouse_get_pos(mlx: Mlx, win: Window, x: *mut c_int, y: *mut c_int) -> c_int;
	pub fn mlx_mouse_move(mlx: Mlx, win: Window, x: c_int, y: c_int) -> c_int;
	pub fn mlx_mouse_hide(mlx: Mlx, win: Window) -> c_int;
	pub fn mlx_mouse_show(mlx: Mlx, win: Window) -> c_int;
	pub fn mlx_get_screen_size(mlx: Mlx, sizex: *mut c_int, sizey: *mut c_int) -> c_int;
}
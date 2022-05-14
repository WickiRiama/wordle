use std::ffi::c_void;
use std::os::raw::{c_int, c_uint, c_ulong};

/// A valid hook.
pub trait Hook: Sized {
	const X_EVENT: c_int;
	const X_MASK: c_int;

	/// Returns the function pointer that will be called by the MLX library to
	/// handle the event.
	fn get_callback<F>() -> usize
	where
		F: FnMut(Self);
}

/// Fires whenever a key is pressed.
#[derive(Debug, Clone, Copy)]
pub struct KeyPress(u32);

impl Hook for KeyPress {
	const X_EVENT: c_int = 2;
	const X_MASK: c_int = 1 << 0;

	fn get_callback<F>() -> usize
	where
		F: FnMut(Self)
	{
		unsafe extern "C" fn callback<F>(keycode: c_ulong, userdata: *mut c_void) -> c_int
		where
			F: FnMut(KeyPress)
		{
			(&mut *(userdata as *mut F))(KeyPress(keycode as u32));
			0
		}
		callback::<F> as usize
	}
}

/// Fires whenever a key is released.
#[derive(Debug, Clone, Copy)]
pub struct KeyRelease(u32);

impl Hook for KeyRelease {
	const X_EVENT: c_int = 3;
	const X_MASK: c_int = 1 << 1;

	fn get_callback<F>() -> usize
	where
		F: FnMut(Self)
	{
		unsafe extern "C" fn callback<F>(keycode: c_ulong, userdata: *mut c_void) -> c_int
		where
			F: FnMut(KeyRelease)
		{
			(&mut *(userdata as *mut F))(KeyRelease(keycode as u32));
			0
		}
		callback::<F> as usize
	}
}

/// Fires whenever a mouse button is pressed.
#[derive(Debug, Clone, Copy)]
pub struct MousePress {
	pub button: u32,
	pub x: i32,
	pub y: i32,
}

impl Hook for MousePress {
	const X_EVENT: c_int = 4;
	const X_MASK: c_int = 1 << 2;

	fn get_callback<F>() -> usize
	where
		F: FnMut(Self)
	{
		unsafe extern "C" fn callback<F>(button: c_uint, x: c_int, y: c_int, userdata: *mut c_void) -> c_int
		where
			F: FnMut(MousePress)
		{
			(&mut *(userdata as *mut F))(MousePress {
				button: button as u32,
				x: x as i32,
				y: y as i32,
			});
			0
		}
		callback::<F> as usize
	}
}

/// Fires whenever a mouse button is released.
#[derive(Debug, Clone, Copy)]
pub struct MouseRelease {
	pub button: u32,
	pub x: i32,
	pub y: i32,
}

impl Hook for MouseRelease {
	const X_EVENT: c_int = 5;
	const X_MASK: c_int = 1 << 3;

	fn get_callback<F>() -> usize
	where
		F: FnMut(Self)
	{
		unsafe extern "C" fn callback<F>(button: c_uint, x: c_int, y: c_int, userdata: *mut c_void) -> c_int
		where
			F: FnMut(MouseRelease)
		{
			(&mut *(userdata as *mut F))(MouseRelease {
				button: button as u32,
				x: x as i32,
				y: y as i32,
			});
			0
		}
		callback::<F> as usize
	}
}

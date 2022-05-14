use std::cell::Cell;
use std::os::raw::c_int;
use std::rc::Rc;
use std::ffi::{CStr, c_void};

use crate::dyn_box::DynBox;
use crate::{Mlx, Image, Hook};

struct Inner<'mlx, 'hooks> {
	mlx: Mlx<'mlx>,
	handle: crate::raw::Window,
	hooks: [Cell<Option<DynBox<'hooks>>>; 36]
}

impl<'mlx, 'hooks> Drop for Inner<'mlx, 'hooks> {
	fn drop(&mut self) {
		unsafe {
			crate::raw::mlx_destroy_window(self.mlx.as_raw(), self.handle);
		}
	}
}

/// An error that might occur whilst creating a window.
#[derive(Debug, Clone, Copy)]
pub struct WindowError;

pub struct Window<'mlx, 'hooks>(Rc<Inner<'mlx, 'hooks>>);

impl<'mlx, 'hooks> Window<'mlx, 'hooks> {
	pub(crate) fn create(mlx: Mlx<'mlx>, width: u32, height: u32, title: &CStr) -> Result<Self, WindowError> {
		let handle = unsafe { crate::raw::mlx_new_window(mlx.as_raw(), width as c_int, height as c_int, title.as_ptr()) };

		if handle.is_null() {
			Err(WindowError)
		} else {
			Ok(Self(Rc::new(Inner {
				handle,
				mlx,
				hooks: unsafe { std::mem::zeroed() },
			})))
		}
	}

	/// Returns the raw handle protected by this instance.
	/// 
	/// ## Safety
	/// 
	/// The returned handle must not be freed.
	#[inline]
	pub unsafe fn as_raw(&self) -> crate::raw::Window {
		self.0 .handle
	}

	/// Returns a reference to the inner `Mlx` instance.
	#[inline]
	pub fn mlx(&self) -> &Mlx<'mlx> {
		&self.0 .mlx
	}

	/// Clears the window.
	#[inline]
	pub fn clear(&self) {
		unsafe { crate::raw::mlx_clear_window(self.mlx().as_raw(), self.as_raw()) };
	}

	/// Puts the pixels of an image on this window.
	#[inline]
	pub fn put_image(&self, img: &Image, x: u32, y: u32) {
		unsafe { crate::raw::mlx_put_image_to_window(self.mlx().as_raw(), self.as_raw(), img.as_raw(), x as c_int, y as c_int) };
	}

	/// Hooks a function to listen for a specific event on this window.
	pub fn hook<H, F>(&self, f: F)
	where
		F: FnMut(H) + 'hooks,
		H: Hook,
	{
		let mut b: Box<F> = Box::new(f);
		unsafe { crate::raw::mlx_hook(self.as_raw(), H::X_EVENT, H::X_MASK, H::get_callback::<F>(), &mut *b as *mut F as *mut c_void) };
		self.0 .hooks[H::X_EVENT as usize].set(Some(DynBox::new(b)));
	}
}
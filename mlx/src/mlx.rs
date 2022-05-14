use std::cell:: Cell;
use std::os::raw::c_int;
use std::rc::Rc;
use std::ffi::{c_void, CStr};

use crate::dyn_box::DynBox;
use crate::{Window, WindowError, ImageError, Image};

struct Inner<'a> {
	handle: crate::raw::Mlx,
	loop_hook: Cell<Option<DynBox<'a>>>,
}

impl<'a> Drop for Inner<'a> {
	fn drop(&mut self) {
		extern "C" {
			fn free(ptr: *mut c_void);
		}

		unsafe {
			// Safety:
			//  We are the only one able to access those resources.
			crate::raw::mlx_destroy_display(self.handle);
			free(self.handle);
		}
	}
}

/// The that is returned whem [`Mlx::init`] fails.
#[derive(Debug, Clone, Copy)]
pub struct InitError;

/// An open connection with *MiniLibX*. An instance of this type is required to work with about anything.
#[derive(Clone)]
pub struct Mlx<'a>(Rc<Inner<'a>>);

impl<'a> Mlx<'a> {
	/// Returns the raw handle protected by this instance.
	/// 
	/// ## Safety
	/// 
	/// The resources associated with that handle must not be freed.
	#[inline]
	pub unsafe fn as_raw(&self) -> crate::raw::Mlx {
		self.0 .handle
	}

    /// Initializes a new instance of *MiniLibX*.
    pub fn init() -> Result<Mlx<'a>, InitError> {
        // Safety:
        //  This function is never unsafe to call.
        let handle = unsafe { crate::raw::mlx_init() };

        if handle.is_null() {
            Err(InitError)
        } else {
            Ok(Mlx(Rc::new(Inner {
				handle,
				loop_hook: Cell::new(None),
			})))
        }
    }

	/// Creates a new [`Window`] instance.
	#[inline]
	pub fn create_window(&self, width: u32, height: u32, name: &CStr) -> Result<Window<'a>, WindowError> {
		Window::create(self.clone(), width, height, name)
	}

	/// Creates a new empty [`Image`].
	#[inline]
	pub fn create_image(&self, width: u32, height: u32) -> Result<Image<'a>, ImageError>  {
		Image::create(self.clone(), width, height)
	}

	/// Creates a new image from the content of an XPM-encoded file.
	#[inline]
	pub fn create_image_from_xpm(&self, xpm_data: &CStr) -> Result<Image<'a>, ImageError> {
		Image::create_from_xpm(self.clone(), xpm_data)
	}

	/// Creates a new image from an XPM-encoded file.
	#[inline]
	pub fn create_image_from_xpm_file(&self, file_path: &CStr) -> Result<Image<'a>, ImageError> {
		Image::create_from_xpm_file(self.clone(), file_path)
	}

	/// Installs a loop hook.
	/// 
	/// The provided function will be called as fast as possible when no events are
	/// available.
	pub fn hook_loop<F>(&self, f: F)
	where
		F: FnMut() + 'a,
	{
		unsafe extern "C" fn callback<F: FnMut()>(userdata: *mut c_void) -> c_int {
			(&mut *(userdata as *mut F))();
			0
		}

		let mut b: Box<F> = Box::new(f);
		unsafe { crate::raw::mlx_loop_hook(self.as_raw(), callback::<F>, &mut *b as *mut F as *mut c_void) };

		self.0.loop_hook.set(Some(DynBox::new(b)));
	}

	/// Loops indefinitely until [`Mlx::stop_loop`] is called.
	#[inline]
	pub fn start_loop(&self) {
		unsafe { crate::raw::mlx_loop(self.as_raw()) };
	}

	/// Stops the currently running loop.
	#[inline]
	pub fn stop_loop(&self) {
		unsafe { crate::raw::mlx_loop_end(self.as_raw()) };
	}
}

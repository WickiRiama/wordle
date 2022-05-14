use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_int;
use std::rc::Rc;

use crate::Mlx;

struct Inner {
	mlx: Mlx,
	handle: crate::raw::Image,
}

impl Drop for Inner {
	fn drop(&mut self) {
		unsafe {
			crate::raw::mlx_destroy_image(self.mlx.as_raw(), self.handle);
		}
	}
}


/// An error that might occur when creating an [`Image`].
#[derive(Debug, Clone, Copy)]
pub struct ImageError;

/// An image, associated with a `width` and a `height`.
/// 
/// An instance of this type is returned by [`Mlx::create_image_from_xpm_file`] and.
pub struct ImageAndSize {
	/// The image itself.
	pub image: Image,
	/// The width of the image, in pixels.
	pub width: u32,
	/// The height of the image, in pixels.
	pub height: u32,
}

/// Stores data about an image.
pub struct ImageData<'a> {
	pub bytes_per_pixel: u32,
	pub line_size: u32,
	pub big_endian: bool,
	pub data: *mut u8,
	_lifetime: PhantomData<&'a ()>,
}

/// A loaded image.
pub struct Image(Rc<Inner>);

impl Image {
	pub(crate) fn create(mlx: Mlx, width: u32, height: u32) -> Result<Self, ImageError> {
		let handle = unsafe { crate::raw::mlx_new_image(mlx.as_raw(), width as c_int, height as c_int) };

		if handle.is_null() {
			Err(ImageError)
		} else {
			Ok(Self(Rc::new(Inner { mlx, handle })))
		}
	}

	pub(crate) fn create_from_xpm(mlx: Mlx, xpmdata: &CStr) -> Result<ImageAndSize, ImageError> {
		let mut width = 0;
		let mut height = 0;

		let handle = unsafe { crate::raw::mlx_xpm_to_image(mlx.as_raw(), xpmdata.as_ptr(), &mut width, &mut height) };

		if handle.is_null() {
			Err(ImageError)
		} else {
			Ok(ImageAndSize {
				image: Self(Rc::new(Inner { mlx, handle })),
				width: width as u32,
				height: width as u32,
			})
		}
	}

	pub(crate) fn create_from_xpm_file(mlx: Mlx, filename: &CStr) -> Result<ImageAndSize, ImageError> {
		let mut width = 0;
		let mut height = 0;

		let handle = unsafe { crate::raw::mlx_xpm_file_to_image(mlx.as_raw(), filename.as_ptr(), &mut width, &mut height) };

		if handle.is_null() {
			Err(ImageError)
		} else {
			Ok(ImageAndSize {
				image: Self(Rc::new(Inner { mlx, handle })),
				width: width as u32,
				height: width as u32,
			})
		}
	}

	/// Returns a reference to the [`Mlx`] instance associated with this [`Image`].
	#[inline]
	pub fn mlx(&self) -> &Mlx {
		&self.0 .mlx
	}
	
	/// Returns the handle protected by this [`Image`] instance.
	/// 
	/// ## Safety
	/// 
	/// The resources referenced by the handle must not be freed.
	#[inline]
	pub unsafe fn as_raw(&self) -> crate::raw::Image {
		self.0 .handle
	}

	/// Returns the address of the image.
	pub fn data_addr(&self) -> ImageData {
		let mut bits_per_pixel = 0;
		let mut line_size = 0;
		let mut endian = 0;

		let data = unsafe { crate::raw::mlx_get_data_addr(self.as_raw(), &mut bits_per_pixel, &mut line_size, &mut endian) };

		ImageData {
			big_endian: endian != 0,
			bytes_per_pixel: bits_per_pixel as u32 / 8,
			line_size: line_size as u32,
			data: data as *mut u8,
			_lifetime: PhantomData,
		}
	}
}
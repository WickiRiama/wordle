use std::ffi::CStr;
use std::os::raw::c_int;
use std::rc::Rc;

use crate::Mlx;

struct Inner<'a> {
	mlx: Mlx<'a>,
	handle: crate::raw::Image,

	width: u32,
	height: u32,
	bytes_per_pixel: u32,
	line_size: u32,
	big_endian: bool,
	data: *mut u8,
}

impl<'a> Drop for Inner<'a> {
	fn drop(&mut self) {
		unsafe {
			crate::raw::mlx_destroy_image(self.mlx.as_raw(), self.handle);
		}
	}
}


/// An error that might occur when creating an [`Image`].
#[derive(Debug, Clone, Copy)]
pub struct ImageError;

/// A loaded image.
#[derive(Clone)]
pub struct Image<'a>(Rc<Inner<'a>>);

impl<'a> Image<'a> {
	pub(crate) fn create(mlx: Mlx<'a>, width: u32, height: u32) -> Result<Self, ImageError> {
		let handle = unsafe { crate::raw::mlx_new_image(mlx.as_raw(), width as c_int, height as c_int) };

		let mut bits_per_pixel = 0;
		let mut line_size = 0;
		let mut endian = 0;

		let data = unsafe { crate::raw::mlx_get_data_addr(handle, &mut bits_per_pixel, &mut line_size, &mut endian) };

		if handle.is_null() {
			Err(ImageError)
		} else {
			Ok(Self(Rc::new(Inner {
				mlx,
				handle,
				width,
				height,
				big_endian: endian != 0,
				bytes_per_pixel: bits_per_pixel as u32 / 8,
				line_size: line_size as u32,
				data: data as *mut u8,
			})))
		}
	}

	pub(crate) fn create_from_xpm(mlx: Mlx<'a>, xpmdata: &CStr) -> Result<Self, ImageError> {
		let mut width = 0;
		let mut height = 0;

		let handle = unsafe { crate::raw::mlx_xpm_to_image(mlx.as_raw(), xpmdata.as_ptr(), &mut width, &mut height) };

		let mut bits_per_pixel = 0;
		let mut line_size = 0;
		let mut endian = 0;

		let data = unsafe { crate::raw::mlx_get_data_addr(handle, &mut bits_per_pixel, &mut line_size, &mut endian) };

		if handle.is_null() {
			Err(ImageError)
		} else {
			Ok(Self(Rc::new(Inner {
				mlx,
				handle,
				width: width as u32,
				height: height as u32,
				big_endian: endian != 0,
				bytes_per_pixel: bits_per_pixel as u32 / 8,
				line_size: line_size as u32,
				data: data as *mut u8,
			})))
		}
	}

	pub(crate) fn create_from_xpm_file(mlx: Mlx<'a>, filename: &CStr) -> Result<Self, ImageError> {
		let mut width = 0;
		let mut height = 0;

		let handle = unsafe { crate::raw::mlx_xpm_file_to_image(mlx.as_raw(), filename.as_ptr(), &mut width, &mut height) };

		let mut bits_per_pixel = 0;
		let mut line_size = 0;
		let mut endian = 0;

		let data = unsafe { crate::raw::mlx_get_data_addr(handle, &mut bits_per_pixel, &mut line_size, &mut endian) };

		if handle.is_null() {
			Err(ImageError)
		} else {
			Ok(Self(Rc::new(Inner {
				mlx,
				handle,
				width: width as u32,
				height: height as u32,
				big_endian: endian != 0,
				bytes_per_pixel: bits_per_pixel as u32 / 8,
				line_size: line_size as u32,
				data: data as *mut u8,
			})))
		}
	}

	/// Returns a reference to the [`Mlx`] instance associated with this [`Image`].
	#[inline]
	pub fn mlx(&self) -> &Mlx<'a> {
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

	#[inline]
	pub fn width(&self) -> u32 {
		self.0 .width
	}

	#[inline]
	pub fn height(&self) -> u32 {
		self.0 .height
	}

	#[inline]
	pub fn bytes_per_pixel(&self) -> u32 {
		self.0 .bytes_per_pixel
	}

	#[inline]
	pub fn line_size(&self) -> u32 {
		self.0 .line_size
	}

	#[inline]
	pub fn is_big_endian(&self) -> bool {
		self.0 .big_endian
	}

	#[inline]
	pub fn data(&self) -> *mut u8 {
		self.0 .data
	}
}
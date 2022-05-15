use std::ffi::CStr;
use std::os::raw::c_int;

use crate::Mlx;

/// An error that might occur when creating an [`Image`].
#[derive(Debug, Clone, Copy)]
pub struct ImageError;

/// A loaded image.
pub struct Image {
    mlx: crate::raw::Mlx,
    handle: crate::raw::Image,

    width: u32,
    height: u32,
    bytes_per_pixel: u32,
    line_size: u32,
    big_endian: bool,
    data: *mut u8,
}

impl Image {
    pub(crate) unsafe fn create(
        mlx: crate::raw::Mlx,
        width: u32,
        height: u32,
    ) -> Result<Self, ImageError> {
        let handle = crate::raw::mlx_new_image(mlx, width as c_int, height as c_int);

        let mut bits_per_pixel = 0;
        let mut line_size = 0;
        let mut endian = 0;

        let data =
            crate::raw::mlx_get_data_addr(handle, &mut bits_per_pixel, &mut line_size, &mut endian);

        if handle.is_null() {
            Err(ImageError)
        } else {
            Ok(Self {
                mlx,
                handle,
                width,
                height,
                big_endian: endian != 0,
                bytes_per_pixel: bits_per_pixel as u32 / 8,
                line_size: line_size as u32,
                data: data as *mut u8,
            })
        }
    }

    pub(crate) unsafe fn create_from_xpm(
        mlx: crate::raw::Mlx,
        xpmdata: &CStr,
    ) -> Result<Self, ImageError> {
        let mut width = 0;
        let mut height = 0;

        let handle = crate::raw::mlx_xpm_to_image(mlx, xpmdata.as_ptr(), &mut width, &mut height);

        if handle.is_null() {
            Err(ImageError)
        } else {
            Ok(Self::image_finish(mlx, handle, width as u32, height as u32))
        }
    }

    pub(crate) unsafe fn create_from_xpm_file(
        mlx: crate::raw::Mlx,
        filename: &CStr,
    ) -> Result<Self, ImageError> {
        let mut width = 0;
        let mut height = 0;

        let handle =
            crate::raw::mlx_xpm_file_to_image(mlx, filename.as_ptr(), &mut width, &mut height);

        if handle.is_null() {
            Err(ImageError)
        } else {
            Ok(Self::image_finish(mlx, handle, width as u32, height as u32))
        }
    }

    unsafe fn image_finish(
        mlx: crate::raw::Mlx,
        handle: crate::raw::Image,
        width: u32,
        height: u32,
    ) -> Self {
        let mut bits_per_pixel = 0;
        let mut line_size = 0;
        let mut endian = 0;

        let data =
            crate::raw::mlx_get_data_addr(handle, &mut bits_per_pixel, &mut line_size, &mut endian);

        Self {
            mlx,
            handle,
            width: width as u32,
            height: height as u32,
            big_endian: endian != 0,
            bytes_per_pixel: bits_per_pixel as u32 / 8,
            line_size: line_size as u32,
            data: data as *mut u8,
        }
    }

    /// Returns a reference to the [`Mlx`] instance associated with this [`Image`].
    #[inline]
    pub fn mlx(&self) -> &Mlx {
        Mlx::wrap_ref(&self.mlx)
    }

    /// Returns the handle protected by this [`Image`] instance.
    ///
    /// ## Safety
    ///
    /// The resources referenced by the handle must not be freed.
    #[inline]
    pub unsafe fn as_raw(&self) -> crate::raw::Image {
        self.handle
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn bytes_per_pixel(&self) -> u32 {
        self.bytes_per_pixel
    }

    #[inline]
    pub fn line_size(&self) -> u32 {
        self.line_size
    }

    #[inline]
    pub fn is_big_endian(&self) -> bool {
        self.big_endian
    }

    #[inline]
    pub fn data(&self) -> *mut u8 {
        self.data
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            crate::raw::mlx_destroy_image(self.mlx, self.handle);
        }
    }
}

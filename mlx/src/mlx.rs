use std::ffi::{c_void, CStr};
use std::os::raw::c_int;

use crate::{Image, ImageError, Window, WindowError};

/// The that is returned whem [`Mlx::init`] fails.
#[derive(Debug, Clone, Copy)]
pub struct InitError;

/// An open connection with *MiniLibX*. An instance of this type is required to work with about anything.
#[repr(transparent)]
pub struct Mlx {
    handle: crate::raw::Mlx,
}

impl Mlx {
    pub(crate) fn wrap_ref(handle: &crate::raw::Mlx) -> &Self {
        unsafe { std::mem::transmute(handle) }
    }

    /// Returns the raw handle protected by this instance.
    ///
    /// ## Safety
    ///
    /// The resources associated with that handle must not be freed.
    #[inline]
    pub unsafe fn as_raw(&self) -> crate::raw::Mlx {
        self.handle
    }

    /// Initializes a new instance of *MiniLibX*.
    pub fn init() -> Result<Mlx, InitError> {
        // Safety:
        //  This function is never unsafe to call.
        let handle = unsafe { crate::raw::mlx_init() };

        if handle.is_null() {
            Err(InitError)
        } else {
            Ok(Mlx {
                handle,
            })
        }
    }

    /// Creates a new [`Window`] instance.
    /// 
    /// ## Safety
    /// 
    /// The produced [`Window`] must be dropped *after* this [`Mlx`] instance.
    #[inline]
    pub unsafe fn create_window(
        &self,
        width: u32,
        height: u32,
        name: &CStr,
    ) -> Result<Window, WindowError> {
        Window::create(self.handle, width, height, name)
    }

    /// Creates a new empty [`Image`].
    /// 
    /// ## Safety
    /// 
    /// The produced [`Image`] must be dropped *after* this [`Mlx`] instance.
    #[inline]
    pub unsafe fn create_image(&self, width: u32, height: u32) -> Result<Image, ImageError> {
        Image::create(self.handle, width, height)
    }

    /// Creates a new image from the content of an XPM-encoded file.
    /// 
    /// ## Safety
    /// 
    /// The produced [`Image`] must be dropped *after* this [`Mlx`] instance.
    #[inline]
    pub unsafe fn create_image_from_xpm(&self, xpm_data: &CStr) -> Result<Image, ImageError> {
        Image::create_from_xpm(self.handle, xpm_data)
    }

    /// Creates a new image from an XPM-encoded file.
    /// 
    /// ## Safety
    /// 
    /// The produced [`Image`] must be dropped *after* this [`Mlx`] instance.
    #[inline]
    pub unsafe fn create_image_from_xpm_file(&self, file_path: &CStr) -> Result<Image, ImageError> {
        Image::create_from_xpm_file(self.handle, file_path)
    }

    /// Loops indefinitely until [`Mlx::stop_loop`] is called.
    pub fn start_loop<'a, F>(&self, mut f: F)
    where
        F: FnMut() + 'a,
    {
        unsafe extern "C" fn callback<F: FnMut()>(userdata: *mut c_void) -> c_int {
            (&mut *(userdata as *mut F))();
            0
        }

        unsafe {
            crate::raw::mlx_loop_hook(
                self.as_raw(),
                callback::<F>,
                &mut f as *mut F as *mut c_void,
            )
        };

        unsafe { crate::raw::mlx_loop(self.as_raw()) };
    }

    /// Stops the currently running loop.
    #[inline]
    pub fn stop_loop(&self) {
        unsafe { crate::raw::mlx_loop_end(self.as_raw()) };
    }

    /// Sets whether key presses should be repeated.
    #[inline]
    pub fn set_autorepeat(&self, yes: bool) {
        if yes {
            unsafe { crate::raw::mlx_do_key_autorepeaton(self.as_raw()) };
        } else {
            unsafe { crate::raw::mlx_do_key_autorepeatoff(self.as_raw()) };
        }
    }
}

impl Drop for Mlx {
    fn drop(&mut self) {
        unsafe {
            // Safety:
            //  We are the only one able to access those resources.
            crate::raw::mlx_destroy_display(self.handle);

            // Safety:
            //  The `Mlx` handle is `malloc`d by MiniLibX itself. We have to
            //  free it ourselves though.
            libc::free(self.handle);
        }
    }
}

use std::ffi::{c_void, CStr};
use std::os::raw::c_int;

use crate::dyn_box::DynBox;
use crate::{Hook, Image, Mlx};

/// An error that might occur whilst creating a window.
#[derive(Debug, Clone, Copy)]
pub struct WindowError;

/// An open window.
pub struct Window {
    mlx: crate::raw::Mlx,
    handle: crate::raw::Window,
}

impl Window {
    pub(crate) unsafe fn create(
        mlx: crate::raw::Mlx,
        width: u32,
        height: u32,
        title: &CStr,
    ) -> Result<Self, WindowError> {
        let handle = 
            crate::raw::mlx_new_window(
                mlx,
                width as c_int,
                height as c_int,
                title.as_ptr(),
            )
        ;

        if handle.is_null() {
            Err(WindowError)
        } else {
            Ok(Self {
                handle,
                mlx,
            })
        }
    }

    /// Returns the raw handle protected by this instance.
    ///
    /// ## Safety
    ///
    /// The returned handle must not be freed.
    #[inline]
    pub unsafe fn as_raw(&self) -> crate::raw::Window {
        self.handle
    }

    /// Returns a reference to the inner `Mlx` instance.
    #[inline]
    pub fn mlx(&self) -> &Mlx {
        Mlx::wrap_ref(&self.mlx)
    }

    /// Clears the window.
    #[inline]
    pub fn clear(&self) {
        unsafe { crate::raw::mlx_clear_window(self.mlx().as_raw(), self.as_raw()) };
    }

    /// Puts the pixels of an image on this window.
    #[inline]
    pub fn put_image(&self, img: &Image, x: u32, y: u32) {
        unsafe {
            crate::raw::mlx_put_image_to_window(
                self.mlx().as_raw(),
                self.as_raw(),
                img.as_raw(),
                x as c_int,
                y as c_int,
            )
        };
    }

    /// Hooks a function to listen for a specific event on this window.
    /// 
    /// ## Safety
    /// 
    /// The produced `DynBox` must be kept alive for the lifetime of the
    /// window.
    pub unsafe fn hook<'a, H, F>(&self, f: F) -> DynBox<'a>
    where
        F: FnMut(H) + 'a,
        H: Hook,
    {
        let mut b: Box<F> = Box::new(f);
        
        crate::raw::mlx_hook(
            self.as_raw(),
            H::X_EVENT,
            H::X_MASK,
            H::get_callback::<F>(),
            &mut *b as *mut F as *mut c_void,
        );
        DynBox::new(b)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            crate::raw::mlx_destroy_window(self.mlx, self.handle);
        }
    }
}
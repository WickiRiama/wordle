use std::marker::PhantomData;
use std::ptr::NonNull;

/// A dynamic box that has an associated lifetime but cannot dowcast.
pub struct DynBox<'a> {
	data: NonNull<()>,
	drop_fn: unsafe fn(data: *mut ()),
	_lifetime: PhantomData<&'a ()>,
}

impl<'a> DynBox<'a> {
	/// Creates a new [`DynBox`] from the provided box.
	pub fn new<T: 'a>(b: Box<T>) -> Self {
		unsafe fn drop<T>(data: *mut ()) {
			Box::from_raw(data as *mut T);
		}

		Self {
			data: unsafe { NonNull::new_unchecked(Box::into_raw(b) as *mut ()) },
			drop_fn: drop::<T>,
			_lifetime: PhantomData,
		}
	}
}

impl<'a> Drop for DynBox<'a> {
	fn drop(&mut self) {
		unsafe {
			(self.drop_fn)(self.data.as_ptr())
		};
	}
}
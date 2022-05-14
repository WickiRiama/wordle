
pub fn set_custom_panic_hook() {
	std::panic::set_hook(Box::new(|info| {
		let s = if let Some(s) = info.payload().downcast_ref::<&str>() {
			*s
		} else if let Some(s) = info.payload().downcast_ref::<String>() {
			s.as_str()
		} else {
			return;
		};

		println!("An unexpected error occured: {}", s);
	}));
}
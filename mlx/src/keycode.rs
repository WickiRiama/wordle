/// A keycode.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct KeyCode(pub u32);

macro_rules! define_keycodes {
	(
		$( const $name:ident = $number:literal; )*		
	) => {
		impl KeyCode {
			$(
				pub const $name: Self = Self($number);
			)*
		}

		impl std::fmt::Debug for KeyCode {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				match *self {
					$(
						Self::$name => f.write_str(stringify!($name)),
					)*
					Self(keycode) => write!(f, "KeyCode({})", keycode),
				}
			}
		}
	};
}

define_keycodes! {
	const A = 97;
	const B = 98;
	const C = 99;
	const D = 100;
	const E = 101;
	const F = 102;
	const G = 103;
	const H = 104;
	const I = 105;
	const J = 106;
	const K = 107;
	const L = 108;
	const M = 109;
	const N = 110;
	const O = 111;
	const P = 112;
	const Q = 113;
	const R = 114;
	const S = 115;
	const T = 116;
	const U = 117;
	const V = 118;
	const W = 119;
	const X = 120;
	const Y = 121;
	const Z = 122;

	const ESCAPE = 65307;
	const BACKSPACE = 65288;
	const RETURN = 65293;
}

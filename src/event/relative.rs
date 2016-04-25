use libc::c_int;
use ffi::*;
use {Event};
use super::{Kind, Code};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Relative {
	All,

	Position(Position),
	Wheel(Wheel),
}

impl Into<Event> for Relative {
	fn into(self) -> Event {
		Event::Relative(self)
	}
}

impl super::Position for Relative { }

impl Kind for Relative {
	fn kind(&self) -> c_int {
		EV_REL
	}
}

impl Code for Relative {
	fn code(&self) -> c_int {
		match self {
			&Relative::All => unreachable!(),

			&Relative::Position(ref v) => v.code(),
			&Relative::Wheel(ref v)    => v.code(),
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(PositionVariants))]
	pub enum Position {
		X,
		Y,
		Z,
		RX,
		RY,
		RZ,
	}
}

impl Into<Event> for Position {
	fn into(self) -> Event {
		Event::Relative(Relative::Position(self))
	}
}

impl super::Position for Position { }

impl Kind for Position {
	fn kind(&self) -> c_int {
		EV_REL
	}
}

impl Code for Position {
	fn code(&self) -> c_int {
		match self {
			&Position::X  => REL_X,
			&Position::Y  => REL_Y,
			&Position::Z  => REL_Z,
			&Position::RX => REL_RX,
			&Position::RY => REL_RY,
			&Position::RZ => REL_RZ,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(WheelVariants))]
	pub enum Wheel {
		Horizontal,
		Dial,
		Vertical,
	}
}

impl Into<Event> for Wheel {
	fn into(self) -> Event {
		Event::Relative(Relative::Wheel(self))
	}
}

impl super::Position for Wheel { }

impl Kind for Wheel {
	fn kind(&self) -> c_int {
		EV_REL
	}
}

impl Code for Wheel {
	fn code(&self) -> c_int {
		match self {
			&Wheel::Horizontal => REL_HWHEEL,
			&Wheel::Dial       => REL_DIAL,
			&Wheel::Vertical   => REL_WHEEL,
		}
	}
}

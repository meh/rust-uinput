use libc::c_int;
use ffi::*;
use {Event};
use super::{Kind, Code, Press, Release};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Controller {
	All,
	Misc(Misc),
	Mouse(Mouse),
	JoyStick(JoyStick),
	GamePad(GamePad),
	Digi(Digi),
	Wheel(Wheel),
	DPad(DPad),
	TriggerHappy(TriggerHappy),
}

impl Into<Event> for Controller {
	fn into(self) -> Event {
		Event::Controller(self)
	}
}

impl Press for Controller { }
impl Release for Controller { }

impl Kind for Controller {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Controller {
	fn code(&self) -> c_int {
		match self {
			&Controller::All => unreachable!(),

			&Controller::Misc(ref v)         => v.code(),
			&Controller::Mouse(ref v)        => v.code(),
			&Controller::JoyStick(ref v)     => v.code(),
			&Controller::GamePad(ref v)      => v.code(),
			&Controller::Digi(ref v)         => v.code(),
			&Controller::Wheel(ref v)        => v.code(),
			&Controller::DPad(ref v)         => v.code(),
			&Controller::TriggerHappy(ref v) => v.code(),
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(MiscVariants))]
	pub enum Misc {
		_0,
		_1,
		_2,
		_3,
		_4,
		_5,
		_6,
		_7,
		_8,
		_9,
	}
}

impl Into<Event> for Misc {
	fn into(self) -> Event {
		Event::Controller(Controller::Misc(self))
	}
}

impl Kind for Misc {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Misc {
	fn code(&self) -> c_int {
		match self {
			&Misc::_0 => BTN_0,
			&Misc::_1 => BTN_1,
			&Misc::_2 => BTN_2,
			&Misc::_3 => BTN_3,
			&Misc::_4 => BTN_4,
			&Misc::_5 => BTN_5,
			&Misc::_6 => BTN_6,
			&Misc::_7 => BTN_7,
			&Misc::_8 => BTN_8,
			&Misc::_9 => BTN_9,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(MouseVariants))]
	pub enum Mouse {
		Left,
		Right,
		Middle,
		Side,
		Extra,
		Forward,
		Back,
		Task,
	}
}

impl Into<Event> for Mouse {
	fn into(self) -> Event {
		Event::Controller(Controller::Mouse(self))
	}
}

impl Code for Mouse {
	fn code(&self) -> c_int {
		match self {
			&Mouse::Left    => BTN_LEFT,
			&Mouse::Right   => BTN_RIGHT,
			&Mouse::Middle  => BTN_MIDDLE,
			&Mouse::Side    => BTN_SIDE,
			&Mouse::Extra   => BTN_EXTRA,
			&Mouse::Forward => BTN_FORWARD,
			&Mouse::Back    => BTN_BACK,
			&Mouse::Task    => BTN_TASK,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(JoyStickVariants))]
	pub enum JoyStick {
		Trigger,
		Thumb,
		Thumb2,
		Top,
		Top2,
		Pinkie,
		Base,
		Base2,
		Base3,
		Base4,
		Base5,
		Base6,
		Dead,
	}
}

impl Into<Event> for JoyStick {
	fn into(self) -> Event {
		Event::Controller(Controller::JoyStick(self))
	}
}

impl Press for JoyStick { }
impl Release for JoyStick { }

impl Kind for JoyStick {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for JoyStick {
	fn code(&self) -> c_int {
		match self {
			&JoyStick::Trigger => BTN_TRIGGER,
			&JoyStick::Thumb   => BTN_THUMB,
			&JoyStick::Thumb2  => BTN_THUMB2,
			&JoyStick::Top     => BTN_TOP,
			&JoyStick::Top2    => BTN_TOP2,
			&JoyStick::Pinkie  => BTN_PINKIE,
			&JoyStick::Base    => BTN_BASE,
			&JoyStick::Base2   => BTN_BASE2,
			&JoyStick::Base3   => BTN_BASE3,
			&JoyStick::Base4   => BTN_BASE4,
			&JoyStick::Base5   => BTN_BASE5,
			&JoyStick::Base6   => BTN_BASE6,
			&JoyStick::Dead    => BTN_DEAD,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(GamePadVariants))]
	pub enum GamePad {
		South,
		A,
		East,
		B,
		C,
		North,
		X,
		West,
		Y,
		Z,
		TL,
		TR,
		TL2,
		TR2,
		Select,
		Start,
		Mode,
		ThumbL,
		ThumbR,
	}
}

impl Into<Event> for GamePad {
	fn into(self) -> Event {
		Event::Controller(Controller::GamePad(self))
	}
}

impl Press for GamePad { }
impl Release for GamePad { }

impl Kind for GamePad {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for GamePad {
	fn code(&self) -> c_int {
		match self {
			&GamePad::South  => BTN_SOUTH,
			&GamePad::A      => BTN_A,
			&GamePad::East   => BTN_EAST,
			&GamePad::B      => BTN_B,
			&GamePad::C      => BTN_C,
			&GamePad::North  => BTN_NORTH,
			&GamePad::X      => BTN_X,
			&GamePad::West   => BTN_WEST,
			&GamePad::Y      => BTN_Y,
			&GamePad::Z      => BTN_Z,
			&GamePad::TL     => BTN_TL,
			&GamePad::TR     => BTN_TR,
			&GamePad::TL2    => BTN_TL2,
			&GamePad::TR2    => BTN_TR2,
			&GamePad::Select => BTN_SELECT,
			&GamePad::Start  => BTN_START,
			&GamePad::Mode   => BTN_MODE,
			&GamePad::ThumbL => BTN_THUMBL,
			&GamePad::ThumbR => BTN_THUMBR,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(DigiVariants))]
	pub enum Digi {
		Pen,
		Rubber,
		Brush,
		Pencil,
		AirBrush,
		Finger,
		Mouse,
		Lens,
		QuintTap,
		Touch,
		Stylus,
		Stylus2,
		DoubleTap,
		TripleTap,
		QuadTap,
	}
}

impl Into<Event> for Digi {
	fn into(self) -> Event {
		Event::Controller(Controller::Digi(self))
	}
}

impl Press for Digi { }
impl Release for Digi { }

impl Kind for Digi {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Digi {
	fn code(&self) -> c_int {
		match self {
			&Digi::Pen       => BTN_TOOL_PEN,
			&Digi::Rubber    => BTN_TOOL_RUBBER,
			&Digi::Brush     => BTN_TOOL_BRUSH,
			&Digi::Pencil    => BTN_TOOL_PENCIL,
			&Digi::AirBrush  => BTN_TOOL_AIRBRUSH,
			&Digi::Finger    => BTN_TOOL_FINGER,
			&Digi::Mouse     => BTN_TOOL_MOUSE,
			&Digi::Lens      => BTN_TOOL_LENS,
			&Digi::QuintTap  => BTN_TOOL_QUINTTAP,
			&Digi::Touch     => BTN_TOUCH,
			&Digi::Stylus    => BTN_STYLUS,
			&Digi::Stylus2   => BTN_STYLUS2,
			&Digi::DoubleTap => BTN_TOOL_DOUBLETAP,
			&Digi::TripleTap => BTN_TOOL_TRIPLETAP,
			&Digi::QuadTap   => BTN_TOOL_QUADTAP,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(WheelVariants))]
	pub enum Wheel {
		GearDown,
		GearUp,
	}
}

impl Into<Event> for Wheel {
	fn into(self) -> Event {
		Event::Controller(Controller::Wheel(self))
	}
}

impl Press for Wheel { }
impl Release for Wheel { }

impl Kind for Wheel {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Wheel {
	fn code(&self) -> c_int {
		match self {
			&Wheel::GearDown => BTN_GEAR_DOWN,
			&Wheel::GearUp   => BTN_GEAR_UP,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(DPadVariants))]
	pub enum DPad {
		Up,
		Down,
		Left,
		Right,
	}
}

impl Into<Event> for DPad {
	fn into(self) -> Event {
		Event::Controller(Controller::DPad(self))
	}
}

impl Press for DPad { }
impl Release for DPad { }

impl Kind for DPad {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for DPad {
	fn code(&self) -> c_int {
		match self {
			&DPad::Up    => BTN_DPAD_UP,
			&DPad::Down  => BTN_DPAD_DOWN,
			&DPad::Left  => BTN_DPAD_LEFT,
			&DPad::Right => BTN_DPAD_RIGHT,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Debug, IterVariants(TriggerHappyVariants))]
	pub enum TriggerHappy {
		_1,
		_2,
		_3,
		_4,
		_5,
		_6,
		_7,
		_8,
		_9,
		_10,
		_11,
		_12,
		_13,
		_14,
		_15,
		_16,
		_17,
		_18,
		_19,
		_20,
		_21,
		_22,
		_23,
		_24,
		_25,
		_26,
		_27,
		_28,
		_29,
		_30,
		_31,
		_32,
		_33,
		_34,
		_35,
		_36,
		_37,
		_38,
		_39,
		_40,
	}
}

impl Into<Event> for TriggerHappy {
	fn into(self) -> Event {
		Event::Controller(Controller::TriggerHappy(self))
	}
}

impl Press for TriggerHappy { }
impl Release for TriggerHappy { }

impl Kind for TriggerHappy {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for TriggerHappy {
	fn code(&self) -> c_int {
		match self {
			&TriggerHappy::_1  => BTN_TRIGGER_HAPPY1,
			&TriggerHappy::_2  => BTN_TRIGGER_HAPPY2,
			&TriggerHappy::_3  => BTN_TRIGGER_HAPPY3,
			&TriggerHappy::_4  => BTN_TRIGGER_HAPPY4,
			&TriggerHappy::_5  => BTN_TRIGGER_HAPPY5,
			&TriggerHappy::_6  => BTN_TRIGGER_HAPPY6,
			&TriggerHappy::_7  => BTN_TRIGGER_HAPPY7,
			&TriggerHappy::_8  => BTN_TRIGGER_HAPPY8,
			&TriggerHappy::_9  => BTN_TRIGGER_HAPPY9,
			&TriggerHappy::_10 => BTN_TRIGGER_HAPPY10,
			&TriggerHappy::_11 => BTN_TRIGGER_HAPPY11,
			&TriggerHappy::_12 => BTN_TRIGGER_HAPPY12,
			&TriggerHappy::_13 => BTN_TRIGGER_HAPPY13,
			&TriggerHappy::_14 => BTN_TRIGGER_HAPPY14,
			&TriggerHappy::_15 => BTN_TRIGGER_HAPPY15,
			&TriggerHappy::_16 => BTN_TRIGGER_HAPPY16,
			&TriggerHappy::_17 => BTN_TRIGGER_HAPPY17,
			&TriggerHappy::_18 => BTN_TRIGGER_HAPPY18,
			&TriggerHappy::_19 => BTN_TRIGGER_HAPPY19,
			&TriggerHappy::_20 => BTN_TRIGGER_HAPPY20,
			&TriggerHappy::_21 => BTN_TRIGGER_HAPPY21,
			&TriggerHappy::_22 => BTN_TRIGGER_HAPPY22,
			&TriggerHappy::_23 => BTN_TRIGGER_HAPPY23,
			&TriggerHappy::_24 => BTN_TRIGGER_HAPPY24,
			&TriggerHappy::_25 => BTN_TRIGGER_HAPPY25,
			&TriggerHappy::_26 => BTN_TRIGGER_HAPPY26,
			&TriggerHappy::_27 => BTN_TRIGGER_HAPPY27,
			&TriggerHappy::_28 => BTN_TRIGGER_HAPPY28,
			&TriggerHappy::_29 => BTN_TRIGGER_HAPPY29,
			&TriggerHappy::_30 => BTN_TRIGGER_HAPPY30,
			&TriggerHappy::_31 => BTN_TRIGGER_HAPPY31,
			&TriggerHappy::_32 => BTN_TRIGGER_HAPPY32,
			&TriggerHappy::_33 => BTN_TRIGGER_HAPPY33,
			&TriggerHappy::_34 => BTN_TRIGGER_HAPPY34,
			&TriggerHappy::_35 => BTN_TRIGGER_HAPPY35,
			&TriggerHappy::_36 => BTN_TRIGGER_HAPPY36,
			&TriggerHappy::_37 => BTN_TRIGGER_HAPPY37,
			&TriggerHappy::_38 => BTN_TRIGGER_HAPPY38,
			&TriggerHappy::_39 => BTN_TRIGGER_HAPPY39,
			&TriggerHappy::_40 => BTN_TRIGGER_HAPPY40,
		}
	}
}

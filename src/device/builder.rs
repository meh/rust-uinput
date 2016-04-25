use std::path::Path;
use std::{mem, slice};
use std::ffi::CString;
use libc::c_int;
use nix::{self, fcntl, unistd, Errno};
use nix::sys::stat;
use ffi::*;
use {Result as Res, Error, Device, Event};
use event::{self, Kind, Code};

#[cfg(feature = "udev")]
use udev;

/// Device builder.
pub struct Builder {
	fd:  c_int,
	def: uinput_user_dev,
	abs: Option<c_int>,
}

impl Builder {
	/// Create a builder from the specified path.
	pub fn open<P: AsRef<Path>>(path: P) -> Res<Self> {
		Ok(Builder {
			fd:  try!(fcntl::open(path.as_ref(), fcntl::O_WRONLY | fcntl::O_NONBLOCK, stat::Mode::empty())),
			def: unsafe { mem::zeroed() },
			abs: None,
		})
	}

	#[cfg(feature = "udev")]
	/// Create a builder from the default path taken from udev.
	pub fn default() -> Res<Self> {
		let     context    = try!(udev::Context::new());
		let mut enumerator = try!(udev::Enumerator::new(&context));

		try!(enumerator.match_subsystem("misc"));
		try!(enumerator.match_sysname("uinput"));

		let device = try!(try!(enumerator.scan_devices())
			.next().ok_or(Error::NotFound));

		Builder::open(try!(device.devnode().ok_or(Error::NotFound)))
	}

	#[cfg(not(feature = "udev"))]
	/// Create a builder from `/dev/uinput`.
	pub fn default() -> Res<Self> {
		Builder::open("/dev/uinput")
	}

	/// Set the name.
	pub fn name<T: AsRef<str>>(mut self, value: T) -> Res<Self> {
		let string = try!(CString::new(value.as_ref()));
		let bytes  = string.as_bytes_with_nul();

		if bytes.len() > UINPUT_MAX_NAME_SIZE as usize {
			try!(Err(nix::Error::from_errno(nix::Errno::EINVAL)));
		}

		(&mut self.def.name)[..bytes.len()]
			.clone_from_slice(unsafe { mem::transmute(bytes) });

		Ok(self)
	}

	/// Set the bus type.
	pub fn bus(mut self, value: u16) -> Self {
		self.def.id.bustype = value;
		self
	}

	/// Set the vendor ID.
	pub fn vendor(mut self, value: u16) -> Self {
		self.def.id.vendor = value;
		self
	}

	/// Set the product ID.
	pub fn product(mut self, value: u16) -> Self {
		self.def.id.product = value;
		self
	}

	/// Set the version.
	pub fn version(mut self, value: u16) -> Self {
		self.def.id.version = value;
		self
	}

	/// Enable the given event.
	pub fn event<T: Into<Event>>(mut self, value: T) -> Res<Self> {
		self.abs = None;

		match value.into() {
			Event::All => {
				try!(self.event(Event::Keyboard(event::Keyboard::All)))
					.event(Event::Controller(event::Controller::All))
			}

			Event::Keyboard(value) => {
				match value {
					event::Keyboard::All => {
						let mut builder = self;

						for item in event::keyboard::Key::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::KeyPad::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Misc::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::InputAssist::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Function::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Braille::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Numeric::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::TouchPad::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Camera::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::keyboard::Attendant::iter_variants() {
							builder = try!(builder.event(item));
						}

						Ok(builder)
					}

					value => {
						unsafe {
							try!(Errno::result(ui_set_evbit(self.fd, value.kind())));
							try!(Errno::result(ui_set_keybit(self.fd, value.code())));
						}

						Ok(self)
					}
				}
			}

			Event::Controller(value) => {
				match value {
					event::Controller::All => {
						let mut builder = self;

						for item in event::controller::Misc::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::Mouse::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::JoyStick::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::GamePad::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::Digi::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::Wheel::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::DPad::iter_variants() {
							builder = try!(builder.event(item));
						}

						for item in event::controller::TriggerHappy::iter_variants() {
							builder = try!(builder.event(item));
						}

						Ok(builder)
					}

					value => {
						unsafe {
							try!(Errno::result(ui_set_evbit(self.fd, value.kind())));
							try!(Errno::result(ui_set_keybit(self.fd, value.code())));
						}

						Ok(self)
					}
				}
			}

			Event::Relative(value) => {
				unsafe {
					try!(Errno::result(ui_set_evbit(self.fd, value.kind())));
					try!(Errno::result(ui_set_relbit(self.fd, value.code())));
				}

				Ok(self)
			}

			Event::Absolute(value) => {
				unsafe {
					try!(Errno::result(ui_set_evbit(self.fd, value.kind())));
					try!(Errno::result(ui_set_absbit(self.fd, value.code())));
				}

				self.abs = Some(value.code());

				Ok(self)
			}
		}
	}

	/// Set the maximum value for the previously enabled absolute event.
	pub fn max(mut self, value: i32) -> Self {
		self.def.absmax[self.abs.unwrap() as usize] = value;
		self
	}

	/// Set the minimum value for the previously enabled absolute event.
	pub fn min(mut self, value: i32) -> Self {
		self.def.absmin[self.abs.unwrap() as usize] = value;
		self
	}

	/// Set the fuzz value for the previously enabled absolute event.
	pub fn fuzz(mut self, value: i32) -> Self {
		self.def.absfuzz[self.abs.unwrap() as usize] = value;
		self
	}

	/// Set the flat value for the previously enabled absolute event.
	pub fn flat(mut self, value: i32) -> Self {
		self.def.absflat[self.abs.unwrap() as usize] = value;
		self
	}

	/// Create the defined device.
	pub fn create(self) -> Res<Device> {
		unsafe {
			let ptr  = &self.def as *const _ as *const u8;
			let size = mem::size_of_val(&self.def);

			try!(unistd::write(self.fd, slice::from_raw_parts(ptr, size)));
			try!(Errno::result(ui_dev_create(self.fd)));
		}

		Ok(Device::new(self.fd))
	}
}

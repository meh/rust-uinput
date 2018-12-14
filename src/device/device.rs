use std::{mem, ptr, slice};
use libc::c_int;
use libc::{timeval, gettimeofday};
use nix::unistd;
use ffi::*;
use {Result as Res, event};
use event::{Kind, Code};

/// The virtual device.
pub struct Device {
	fd: c_int,
}

impl Device {
	/// Wrap a file descriptor in a `Device`.
	pub fn new(fd: c_int) -> Self {
		Device {
			fd: fd
		}
	}

	#[doc(hidden)]
	pub fn write(&self, kind: c_int, code: c_int, value: c_int) -> Res<()> {
		unsafe {
			let mut event = input_event {
				time:  timeval { tv_sec: 0, tv_usec: 0 },
				kind:  kind as u16,
				code:  code as u16,
				value: value as i32,
			};

			gettimeofday(&mut event.time, ptr::null_mut());

			let ptr  = &event as *const _ as *const u8;
			let size = mem::size_of_val(&event);

			try!(unistd::write(self.fd, slice::from_raw_parts(ptr, size)));
		}

		Ok(())
	}

	/// Synchronize the device.
	pub fn synchronize(&self) -> Res<()> {
		self.write(EV_SYN, SYN_REPORT, 0)
	}

	/// Send an event.
	pub fn send<T: Into<event::Event>>(&self, event: T, value: i32) -> Res<()> {
		let event = event.into();
		self.write(event.kind(), event.code(), value)
	}

	/// Send a press event.
	pub fn press<T: event::Press>(&self, event: &T) -> Res<()> {
		self.write(event.kind(), event.code(), 1)
	}

	/// Send a release event.
	pub fn release<T: event::Release>(&self, event: &T) -> Res<()> {
		self.write(event.kind(), event.code(), 0)
	}

	/// Send a press and release event.
	pub fn click<T: event::Press + event::Release>(&self, event: &T) -> Res<()> {
		try!(self.press(event));
		try!(self.release(event));

		Ok(())
	}

	/// Send a relative or absolute positioning event.
	pub fn position<T: event::Position>(&self, event: &T, value: i32) -> Res<()> {
		self.write(event.kind(), event.code(), value)
	}
}

impl Drop for Device {
	fn drop(&mut self) {
		unsafe {
			ui_dev_destroy(self.fd);
		}
	}
}

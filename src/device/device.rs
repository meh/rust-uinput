use std::{mem, ptr, slice};
use libc::c_int;
use libc::{timeval, gettimeofday};
use nix::unistd;
use ffi::*;
use {Result as Res, Error, Event, event};

pub struct Device {
	fd: c_int,
}

impl Device {
	pub fn new(fd: c_int) -> Self {
		Device {
			fd: fd
		}
	}

	fn write(&mut self, kind: c_int, code: c_int, value: c_int) -> Res<()> {
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

	pub fn synchronize(&mut self) -> Res<()> {
		self.write(EV_SYN, SYN_REPORT, 0)
	}

	pub fn press<T: event::Press>(&mut self, event: &T) -> Res<()> {
		self.write(event.kind(), event.code(), 1)
	}

	pub fn release<T: event::Release>(&mut self, event: &T) -> Res<()> {
		self.write(event.kind(), event.code(), 0)
	}

	pub fn click<T: event::Press + event::Release>(&mut self, event: &T) -> Res<()> {
		try!(self.press(event));
		try!(self.release(event));

		Ok(())
	}

	pub fn position<T: event::Position>(&mut self, event: &T, value: i32) -> Res<()> {
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

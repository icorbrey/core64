use crate::mem::{addr::VI_STATUS_REG, io};

use super::status::Status;

pub fn get_status() -> Status {
    Status::from(io::read(VI_STATUS_REG))
}

pub fn set_status(status: Status) {
    io::write(VI_STATUS_REG, status.into());
}

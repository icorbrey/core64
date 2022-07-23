use crate::mem::addr::*;
use crate::mem::io;

use super::status::Status;

pub fn get_status() -> Status {
    Status::from(io::read(VI_STATUS_REG))
}

pub fn set_status(status: Status) {
    io::write(VI_STATUS_REG, status.into());
}

pub fn get_dram_address() -> u32 {
    io::read(VI_DRAM_ADDR_REG)
}

pub fn set_dram_address(address: u32) {
    io::write(VI_DRAM_ADDR_REG, address);
}

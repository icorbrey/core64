use crate::mem::addr::*;
use crate::mem::io;
use crate::utils::bytes;

use super::horizontal_sync::HorizontalSync;
use super::status::Status;
use super::video_timing::VideoTiming;

pub fn get_status() -> Status {
    Status::from(io::read(VI_STATUS_REG))
}

pub fn set_status(status: Status) {
    io::write(VI_STATUS_REG, status.into());
}

pub fn get_dram_address() -> u32 {
    bytes::to_u32(0, 24, io::read(VI_DRAM_ADDR_REG))
}

pub fn set_dram_address(address: u32) {
    io::write(VI_DRAM_ADDR_REG, bytes::from_u32(0, 24, address));
}

pub fn get_frame_buffer_line_width() -> u32 {
    bytes::to_u32(0, 11, io::read(VI_H_WIDTH_REG))
}

pub fn set_frame_buffer_line_width(pixels: u32) {
    io::write(VI_H_WIDTH_REG, bytes::from_u32(0, 11, pixels));
}

pub fn get_vertical_interrupt() -> u32 {
    bytes::to_u32(0, 9, io::read(VI_V_INTR_REG))
}

pub fn set_vertical_interrupt(interrupt: u32) {
    io::write(VI_V_INTR_REG, bytes::from_u32(0, 9, interrupt));
}

pub fn get_current_halfline() -> u32 {
    bytes::to_u32(0, 9, io::read(VI_V_CURRENT_LINE_REG))
}

pub fn clear_vertical_interrupt() {
    io::write(VI_V_CURRENT_LINE_REG, 0x0000);
}

pub fn get_video_timing() -> VideoTiming {
    VideoTiming::from(io::read(VI_TIMING_REG))
}

pub fn set_video_timing(video_timing: VideoTiming) {
    io::write(VI_TIMING_REG, video_timing.into())
}

pub fn get_vertical_sync() -> u32 {
    bytes::to_u32(0, 9, io::read(VI_V_SYNC_REG))
}

pub fn set_vertical_sync(vertical_sync: u32) {
    io::write(VI_V_SYNC_REG, bytes::from_u32(0, 9, vertical_sync));
}

pub fn get_horizontal_sync() -> HorizontalSync {
    HorizontalSync::from(io::read(VI_H_SYNC_REG))
}

pub fn set_horizontal_sync(horizontal_sync: HorizontalSync) {
    io::write(VI_H_SYNC_REG, horizontal_sync.into());
}

pub fn get_horizontal_sync_leap() -> u32 {
    bytes::to_u32(0, 12, io::read(VI_H_SYNC_LEAP_REG))
}

pub fn set_horizontal_sync_leap(horizontal_sync_leap: u32) {
    io::write(
        VI_H_SYNC_LEAP_REG,
        bytes::from_u32(0, 12, horizontal_sync_leap),
    );
}

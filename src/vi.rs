pub mod anti_aliasing_mode;
pub mod color_burst;
pub mod horizontal_sync;
pub mod horizontal_video;
pub mod interface_type;
pub mod vertical_video;
pub mod video_mode;
pub mod video_scale;
pub mod video_settings;
pub mod video_timing;

use crate::mem::addr::*;
use crate::mem::io;
use crate::utils::bytes;

use self::color_burst::ColorBurst;
use self::horizontal_sync::HorizontalSync;
use self::horizontal_video::HorizontalVideo;
use self::vertical_video::VerticalVideo;
use self::video_mode::VideoMode;
use self::video_scale::VideoScale;
use self::video_settings::VideoSettings;
use self::video_timing::VideoTiming;

/// Gets the system's video settings.
pub fn get_video_settings() -> VideoSettings {
    VideoSettings::from(io::read(VI_STATUS_REG))
}

/// Sets the system's video settings.
///
/// ## Arguments
///
/// * `settings` - The video settings to save.
pub fn set_video_settings(settings: VideoSettings) {
    io::write(VI_STATUS_REG, settings.into());
}

/// Gets the current frame buffer's address from DRAM.
pub fn get_frame_buffer_address() -> usize {
    bytes::to_u32(0, 24, io::read(VI_DRAM_ADDR_REG)) as usize
}

/// Assigns a new frame buffer's address tp DRAM.
///
/// ## Arguments
///
/// * `address` - The new frame buffer's address.
pub fn set_frame_buffer_address(address: usize) {
    io::write(VI_DRAM_ADDR_REG, bytes::from_u32(0, 24, address as u32));
}

/// Gets the frame buffer's line width in pixels.
pub fn get_frame_buffer_line_width() -> u32 {
    bytes::to_u32(0, 11, io::read(VI_H_WIDTH_REG))
}

/// Sets the frame buffer's line width in pixels.
///
/// ## Arguments
///
/// * `pixels` - The new frame buffer line width.
pub fn set_frame_buffer_line_width(pixels: u32) {
    io::write(VI_H_WIDTH_REG, bytes::from_u32(0, 11, pixels));
}

/// Gets the current vertical interrupt point.
pub fn get_vertical_interrupt() -> u32 {
    bytes::to_u32(0, 9, io::read(VI_V_INTR_REG))
}

/// Sets the vertical interrupt point.
///
/// ## Arguments
///
/// * `interrupt` - The vertical interrupt point.
pub fn set_vertical_interrupt(interrupt: u32) {
    io::write(VI_V_INTR_REG, bytes::from_u32(0, 9, interrupt));
}

/// Gets the current vertical halfline.
pub fn get_current_vertical_halfline() -> u32 {
    bytes::to_u32(0, 9, io::read(VI_V_CURRENT_LINE_REG))
}

/// Clears the vertical interrupt point.
pub fn clear_vertical_interrupt() {
    io::write(VI_V_CURRENT_LINE_REG, 0x0000);
}

/// Gets the system's video timing information.
pub fn get_video_timing() -> VideoTiming {
    VideoTiming::from(io::read(VI_TIMING_REG))
}

/// Sets new video timing information.
///
/// ## Arguments
///
/// * `video_timing` - The new video timing information.
pub fn set_video_timing(video_timing: VideoTiming) {
    io::write(VI_TIMING_REG, video_timing.into())
}

/// Gets the vertical sync, measured in half-lines per field.
pub fn get_vertical_sync() -> u32 {
    bytes::to_u32(0, 9, io::read(VI_V_SYNC_REG))
}

/// Sets the vertical sync
///
/// ## Arguments
///
/// * `sync` - The new vertical sync, measured in half-lines per field.
pub fn set_vertical_sync(sync: u32) {
    io::write(VI_V_SYNC_REG, bytes::from_u32(0, 9, sync));
}

/// Gets the horizontal sync.
pub fn get_horizontal_sync() -> HorizontalSync {
    HorizontalSync::from(io::read(VI_H_SYNC_REG))
}

/// Sets the horizontal sync information.
///
/// ## Arguments
///
/// * `sync` - The new horizontal sync information.
pub fn set_horizontal_sync(sync: HorizontalSync) {
    io::write(VI_H_SYNC_REG, sync.into());
}

/// Gets the horizontal sync leap.
pub fn get_horizontal_sync_leap() -> u32 {
    bytes::to_u32(0, 12, io::read(VI_H_SYNC_LEAP_REG))
}

/// Sets the horizontal sync leap.
///
/// ## Arguments
///
/// * `sync_leap` - The new horizontal sync leap.
pub fn set_horizontal_sync_leap(sync_leap: u32) {
    io::write(VI_H_SYNC_LEAP_REG, bytes::from_u32(0, 12, sync_leap));
}

/// Gets the horizontal video data.
pub fn get_horizontal_video_data() -> HorizontalVideo {
    HorizontalVideo::from(io::read(VI_H_VIDEO_REG))
}

/// Sets the horizontal video data.
///
/// ## Arguments
///
/// - `video_data` - The new horizontal video data.
pub fn set_horizontal_video_data(video_data: HorizontalVideo) {
    io::write(VI_H_VIDEO_REG, video_data.into());
}

/// Gets the vertical video data.
pub fn get_vertical_video_data() -> VerticalVideo {
    VerticalVideo::from(io::read(VI_V_VIDEO_REG))
}

/// Sets the vertical video data.
///
/// ## Arguments
///
/// - `video_data` - The new vertical video data.
pub fn set_vertical_video_data(video_data: VerticalVideo) {
    io::write(VI_V_VIDEO_REG, video_data.into())
}

/// Gets the system's color burst data.
pub fn get_color_burst_data() -> ColorBurst {
    ColorBurst::from(io::read(VI_V_BURST_REG))
}

/// Sets new color burst data.
///
/// ## Arguments
///
/// * `burst_data` - The new color burst data.
pub fn set_color_burst_data(burst_data: ColorBurst) {
    io::write(VI_V_BURST_REG, burst_data.into())
}

/// Gets the system's horizontal video scale.
pub fn get_horizontal_scale() -> VideoScale {
    VideoScale::from(io::read(VI_X_SCALE_REG))
}

/// Sets the system's horizontal video scale.
///
/// ## Arguments
///
/// * `scale` - The new video scale.
pub fn set_horizontal_scale(scale: VideoScale) {
    io::write(VI_X_SCALE_REG, scale.into())
}

/// Gets the system's vertical video scale.
pub fn get_vertical_scale() -> VideoScale {
    VideoScale::from(io::read(VI_Y_SCALE_REG))
}

/// Sets the system's vertical video scale.
///
/// ## Arguments
///
/// * `scale` - The new video scale.
pub fn set_vertical_scale(scale: VideoScale) {
    io::write(VI_Y_SCALE_REG, scale.into())
}

/// Gets the system's video mode.
pub fn get_video_mode() -> VideoMode {
    VideoMode::from(io::read(VIDEO_MODE))
}

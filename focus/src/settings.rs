use crate::color::*;
use crate::enums::{LedMode, WirelessPowerMode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub keymap_custom: Vec<u16>,
    pub keymap_default: Vec<u16>,
    pub keymap_only_custom: bool,
    pub settings_default_layer: u8,
    pub superkeys_map: Vec<u16>,
    pub superkeys_wait_for: Duration,
    pub superkeys_timeout: Duration,
    pub superkeys_repeat: Duration,
    pub superkeys_hold_start: Duration,
    pub superkeys_overlap: u8,
    pub led_mode: LedMode,
    pub led_brightness_top: u8,
    pub led_brightness_underglow: Option<u8>,
    pub led_brightness_wireless_top: Option<u8>,
    pub led_brightness_wireless_underglow: Option<u8>,
    pub led_fade: Option<u16>,
    pub led_theme: Vec<RGB>,
    pub palette: Vec<RGBA>,
    pub color_map: Vec<u8>,
    pub led_idle_true_sleep: Option<bool>,
    pub led_idle_true_sleep_time: Option<Duration>,
    pub led_idle_time_limit: Duration,
    pub led_idle_wireless: Option<bool>,
    pub qukeys_hold_timeout: Duration,
    pub qukeys_overlap_threshold: Duration,
    pub macros_map: Vec<u8>,
    pub mouse_speed: u8,
    pub mouse_delay: Duration,
    pub mouse_acceleration_speed: u8,
    pub mouse_acceleration_delay: Duration,
    pub mouse_wheel_speed: u8,
    pub mouse_wheel_delay: Duration,
    pub mouse_speed_limit: u8,
    pub wireless_battery_saving_mode: Option<bool>,
    pub wireless_rf_power_level: Option<WirelessPowerMode>,
    pub wireless_rf_channel_hop: Option<bool>,
}

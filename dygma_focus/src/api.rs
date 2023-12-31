use crate::prelude::*;
use crate::{Focus, MAX_LAYERS};
use anyhow::{anyhow, bail, Result};
use log::trace;
use std::str::FromStr;
use std::time::Duration;

/// Private methods
impl Focus {
    /// Sends a command to the device, with no response.
    fn command(&mut self, command: &str) -> Result<()> {
        trace!("Command TX: {}", command);

        self.serial.write_all(format!("{}\n", command).as_bytes())?;

        Ok(())
    }

    /// Sends a command to the device, and returns the response as a string.
    fn command_response_string(&mut self, command: &str) -> Result<String> {
        self.command(command)?;

        let eof_marker = b"\r\n.\r\n";

        self.response_buffer.clear();

        loop {
            let prev_len = self.response_buffer.len();
            self.response_buffer.resize(prev_len + 1024, 0);
            match self.serial.read(&mut self.response_buffer[prev_len..]) {
                Ok(0) => continue,
                Ok(size) => {
                    self.response_buffer.truncate(prev_len + size);

                    if self.response_buffer.ends_with(eof_marker) {
                        break;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => bail!("Error reading from serial port: {:?}", e),
            }
        }

        while let Some(pos) = self
            .response_buffer
            .windows(eof_marker.len())
            .position(|window| window == eof_marker)
        {
            self.response_buffer.drain(pos..pos + eof_marker.len());
        }

        let start = self
            .response_buffer
            .iter()
            .position(|&b| !b.is_ascii_whitespace())
            .unwrap_or(0);

        let end = self
            .response_buffer
            .iter()
            .rposition(|&b| !b.is_ascii_whitespace())
            .map_or(0, |p| p + 1);

        let trimmed_buffer = &self.response_buffer[start..end];

        let response = std::str::from_utf8(trimmed_buffer)
            .map_err(|e| anyhow!("Failed to convert response to UTF-8 string: {:?}", e))?;

        trace!("Command RX: {}", &response);

        Ok(response.to_string())
    }

    /// Sends a command to the device, and returns the response as a numerical value.
    fn command_response_numerical<T>(&mut self, command: &str) -> Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let response = self.command_response_string(command)?;
        response
            .parse::<T>()
            .map_err(|e| anyhow!("Failed to parse response: {:?}", e))
    }

    /// Sends a command to the device, and returns the response as a duration.
    fn command_response_duration(
        &mut self,
        command: &str,
        time_unit: TimeUnit,
    ) -> Result<Duration> {
        let response = self.command_response_numerical(command)?;

        let duration = match time_unit {
            TimeUnit::Milliseconds => Duration::from_millis(response),
            TimeUnit::Seconds => Duration::from_secs(response),
        };

        Ok(duration)
    }

    /// Sends a command to the device, and returns the response as a boolean value.
    fn command_response_bool(&mut self, command: &str) -> Result<bool> {
        let response = self.command_response_string(command)?;
        Ok(response == "1" || response == "true")
    }

    /// Sends a command to the device, and returns the response as a vector of strings.
    fn command_response_vec_string(&mut self, command: &str) -> Result<Vec<String>> {
        Ok(self
            .command_response_string(command)?
            .lines()
            .map(|line| line.replace('\r', ""))
            .collect())
    }
}

/// Public methods
impl Focus {
    /// Gets the whole configuration stored in the keyboard.
    pub fn dygma_backup(&mut self) -> Result<Configuration> {
        Ok(Configuration {
            keymap_custom: self.keymap_custom_get()?,
            keymap_default: self.keymap_default_get()?,
            keymap_only_custom: self.keymap_only_custom_get()?,
            settings_default_layer: self.settings_default_layer_get()?,
            settings_version: self.settings_version_get()?,
            // eeprom_contents: self.eeprom_contents_get()?,
            superkeys_map: self.superkeys_map_get()?,
            superkeys_wait_for: self.superkeys_wait_for_get()?,
            superkeys_timeout: self.superkeys_timeout_get()?,
            superkeys_repeat: self.superkeys_repeat_get()?,
            superkeys_hold_start: self.superkeys_hold_start_get()?,
            superkeys_overlap: self.superkeys_overlap_get()?,
            led_mode: self.led_mode_get()?,
            led_brightness: self.led_brightness_get()?,
            led_brightness_underglow: self.led_brightness_underglow_get()?,
            led_brightness_wireless: self.led_brightness_wireless_get()?,
            led_brightness_underglow_wireless: self.led_brightness_underglow_wireless_get()?,
            led_fade: self.led_fade_get()?,
            led_theme: self.led_theme_get()?,
            palette: self.palette_get()?,
            color_map: self.color_map_get()?,
            led_idle_true_sleep: self.led_idle_true_sleep_get()?,
            led_idle_true_sleep_time: self.led_idle_true_sleep_time_get()?,
            led_idle_time_limit: self.led_idle_time_limit_get()?,
            led_idle_wireless: self.led_idle_wireless_get()?,
            hardware_version: self.hardware_version_get()?,
            qukeys_hold_timeout: self.qukeys_hold_timeout_get()?,
            qukeys_overlap_threshold: self.qukeys_overlap_threshold_get()?,
            macros_map: self.macros_map_get()?,
            mouse_speed: self.mouse_speed_get()?,
            mouse_delay: self.mouse_delay_get()?,
            mouse_acceleration_speed: self.mouse_acceleration_speed_get()?,
            mouse_acceleration_delay: self.mouse_acceleration_delay_get()?,
            mouse_wheel_speed: self.mouse_wheel_speed_get()?,
            mouse_wheel_delay: self.mouse_wheel_delay_get()?,
            mouse_speed_limit: self.mouse_speed_limit_get()?,
            wireless_battery_saving_mode: self.wireless_battery_saving_mode_get()?,
            wireless_rf_power_level: self.wireless_rf_power_level_get()?,
            wireless_rf_channel_hop: self.wireless_rf_channel_hop_get()?,
        })
    }

    /// Sets the whole configuration stored in the keyboard.
    pub fn dygma_restore(&mut self, config: &Configuration) -> Result<()> {
        self.keymap_custom_set(&config.keymap_custom)?;
        self.keymap_default_set(&config.keymap_default)?;
        self.keymap_only_custom_set(config.keymap_only_custom)?;
        self.settings_default_layer_set(config.settings_default_layer)?;
        self.settings_version_set(&config.settings_version)?;
        // self.eeprom_contents_set(&config.eeprom_contents)?;
        self.superkeys_map_set(&config.superkeys_map)?;
        self.superkeys_wait_for_set(config.superkeys_wait_for)?;
        self.superkeys_timeout_set(config.superkeys_timeout)?;
        self.superkeys_repeat_set(config.superkeys_repeat)?;
        self.superkeys_hold_start_set(config.superkeys_hold_start)?;
        self.superkeys_overlap_set(config.superkeys_overlap)?;
        self.led_mode_set(config.led_mode)?;
        self.led_brightness_set(config.led_brightness)?;
        self.led_brightness_underglow_set(config.led_brightness_underglow)?;
        self.led_brightness_wireless_set(config.led_brightness_wireless)?;
        self.led_brightness_underglow_wireless_set(config.led_brightness_underglow_wireless)?;
        self.led_fade_set(config.led_fade)?;
        self.led_theme_set(&config.led_theme)?;
        self.palette_set(&config.palette)?;
        self.color_map_set(&config.color_map)?;
        self.led_idle_true_sleep_set(config.led_idle_true_sleep)?;
        self.led_idle_true_sleep_time_set(config.led_idle_true_sleep_time)?;
        self.led_idle_time_limit_set(config.led_idle_time_limit)?;
        self.led_idle_wireless_set(config.led_idle_wireless)?;
        self.hardware_version_set(&config.hardware_version)?;
        self.qukeys_hold_timeout_set(config.qukeys_hold_timeout)?;
        self.qukeys_overlap_threshold_set(config.qukeys_overlap_threshold)?;
        self.macros_map_set(&config.macros_map)?;
        self.mouse_speed_set(config.mouse_speed)?;
        self.mouse_delay_set(config.mouse_delay)?;
        self.mouse_acceleration_speed_set(config.mouse_acceleration_speed)?;
        self.mouse_acceleration_delay_set(config.mouse_acceleration_delay)?;
        self.mouse_wheel_speed_set(config.mouse_wheel_speed)?;
        self.mouse_wheel_delay_set(config.mouse_wheel_delay)?;
        self.mouse_speed_limit_set(config.mouse_speed_limit)?;
        self.wireless_battery_saving_mode_set(config.wireless_battery_saving_mode)?;
        self.wireless_rf_power_level_set(config.wireless_rf_power_level)?;
        self.wireless_rf_channel_hop_set(config.wireless_rf_channel_hop)?;

        Ok(())
    }
}

/// Public API methods
impl Focus {
    /// Get the version of the firmware.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#version
    pub fn version_get(&mut self) -> Result<String> {
        self.command_response_string("version")
    }

    /// Gets the whole custom keymap stored in the keyboard.
    ///
    /// Layers 0 and above, The layers are -1 to Bazecor.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#keymapcustom
    pub fn keymap_custom_get(&mut self) -> Result<String> {
        self.command_response_string("keymap.custom")
    }

    /// Sets the whole custom keymap stored in the keyboard.
    ///
    /// Layers 0 and above, The layers are -1 to Bazecor.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#keymapcustom
    pub fn keymap_custom_set(&mut self, data: &str) -> Result<()> {
        if self.keymap_custom_get()? == data {
            return Ok(());
        }

        self.command(&format!("keymap.custom {}", data))
    }

    /// Gets the default keymap stored in the keyboard.
    ///
    /// Layers -1 and -2, the layers are -1 to Bazecor.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#keymapdefault
    pub fn keymap_default_get(&mut self) -> Result<String> {
        self.command_response_string("keymap.default")
    }

    /// Sets the default keymap stored in the keyboard.
    ///
    /// Layers -1 and -2, the layers are -1 to Bazecor.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#keymapdefault
    pub fn keymap_default_set(&mut self, data: &str) -> Result<()> {
        if self.keymap_default_get()? == data {
            return Ok(());
        }

        self.command(&format!("keymap.default {}", data))
    }

    /// Gets the user setting of hiding the default layers.
    ///
    /// It does not allow you to increment the number of available layers by start using the default ones.
    /// They are there so you can store a backup for two layers in your keyboard.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#keymaponlycustom
    pub fn keymap_only_custom_get(&mut self) -> Result<bool> {
        self.command_response_bool("keymap.onlyCustom")
    }

    /// Sets the user setting of hiding the default layers.
    ///
    /// It does not allow you to increment the number of available layers by start using the default ones.
    /// They are there so you can store a backup for two layers in your keyboard.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#keymaponlycustom
    pub fn keymap_only_custom_set(&mut self, state: bool) -> Result<()> {
        if self.keymap_only_custom_get()? == state {
            return Ok(());
        }

        self.command(&format!("keymap.onlyCustom {}", state as u8))
    }

    /// Gets the default layer the keyboard will boot with.
    ///
    /// The layer is -1 to Bazecor.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#settingsdefaultlayer
    pub fn settings_default_layer_get(&mut self) -> Result<u8> {
        self.command_response_numerical("settings.defaultLayer")
    }

    /// Sets the default layer the keyboard will boot with.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#settingsdefaultlayer
    pub fn settings_default_layer_set(&mut self, layer: u8) -> Result<()> {
        if layer > MAX_LAYERS {
            bail!("Layer out of range, max is {}: {}", MAX_LAYERS, layer);
        }

        if self.settings_default_layer_get()? == layer {
            return Ok(());
        }

        self.command(&format!("settings.defaultLayer {}", layer))
    }

    /// Gets a boolean value that states true if all checks have been performed on the current settings, and its upload was done in the intended way.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#settingsvalid
    pub fn settings_valid_get(&mut self) -> Result<bool> {
        self.command_response_numerical("settings.valid?")
    }

    /// Gets the current settings version.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#settingsversion
    pub fn settings_version_get(&mut self) -> Result<String> {
        self.command_response_string("settings.version")
    }

    /// Sets the current settings version.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#settingsversion
    pub fn settings_version_set(&mut self, version: &str) -> Result<()> {
        if self.settings_version_get()? == version {
            return Ok(());
        }

        self.command(&format!("settings.version {}", version))
    }

    /// Gets the CRC checksum of the layout.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#settingscrc
    pub fn settings_crc_get(&mut self) -> Result<String> {
        self.command_response_string("settings.crc")
    }

    /// Gets the EEPROM's contents.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#eepromcontents
    pub fn eeprom_contents_get(&mut self) -> Result<String> {
        self.command_response_string("eeprom.contents")
    }

    /// Sets the EEPROM's contents.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#eepromcontents
    pub fn eeprom_contents_set(&mut self, data: &str) -> Result<()> {
        if self.eeprom_contents_get()? == data {
            return Ok(());
        }

        self.command(&format!("eeprom.contents {}", data))
    }

    /// Gets the EEPROM's free bytes.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#eepromfree
    pub fn eeprom_free_get(&mut self) -> Result<String> {
        self.command_response_string("eeprom.free")
    }

    // TODO: upgrade.start
    // TODO: upgrade.neuron
    // TODO: upgrade.end
    // TODO: upgrade.keyscanner.isConnected
    // TODO: upgrade.keyscanner.isBootloader
    // TODO: upgrade.keyscanner.begin
    // TODO: upgrade.keyscanner.isReady
    // TODO: upgrade.keyscanner.getInfo
    // TODO: upgrade.keyscanner.sendWrite
    // TODO: upgrade.keyscanner.validate
    // TODO: upgrade.keyscanner.finish
    // TODO: upgrade.keyscanner.sendStart

    /// Gets the Superkeys map.
    ///
    /// Each action in a Superkey is represented by a key code number that encodes the action, for example if you use the number 44, you are encoding space, etc...
    ///
    /// To know more about keycodes and to find the right one for your actions, check the key map database.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeysmap
    pub fn superkeys_map_get(&mut self) -> Result<String> {
        self.command_response_string("superkeys.map")
    }

    /// Sets the Superkeys map.
    ///
    /// Each action in a Superkey is represented by a key code number that encodes the action, for example if you use the number 44, you are encoding space, etc...
    ///
    /// To know more about keycodes and to find the right one for your actions, check the key map database.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeysmap
    pub fn superkeys_map_set(&mut self, data: &str) -> Result<()> {
        if self.superkeys_map_get()? == data {
            return Ok(());
        }

        self.command(&format!("superkeys.map {}", data))
    }

    /// Gets the Superkeys wait for duration.
    ///
    /// Wait for value specifies the time between the first and subsequent releases of the HOLD actions meanwhile is held,
    ///
    /// So for example,
    /// if the variable is set to 500ms, you can maintain the hold key, it will emmit a key code corresponding to the action that it triggers,
    /// then it will wait for wait for time for making another key press with that same key code.
    /// This enables the user to delay the hold "machinegun" to be able to release the key and achieve a single keypress from a hold action.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeyswaitfor
    pub fn superkeys_wait_for_get(&mut self) -> Result<Duration> {
        self.command_response_duration("superkeys.waitfor", TimeUnit::Milliseconds)
    }

    /// Sets the Superkeys wait for duration.
    ///
    /// Wait for value specifies the time between the first and subsequent releases of the HOLD actions meanwhile is held,
    ///
    /// So for example,
    /// if the variable is set to 500ms, you can maintain the hold key, it will emmit a key code corresponding to the action that it triggers,
    /// then it will wait for wait for time for making another key press with that same key code.
    /// This enables the user to delay the hold "machinegun" to be able to release the key and achieve a single keypress from a hold action.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeyswaitfor
    pub fn superkeys_wait_for_set(&mut self, duration: Duration) -> Result<()> {
        if self.superkeys_wait_for_get()? == duration {
            return Ok(());
        }

        self.command(&format!("superkeys.waitfor {}", &duration.as_millis()))
    }

    /// Gets the Superkeys timeout of how long it waits for the next tap.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeystimeout
    pub fn superkeys_timeout_get(&mut self) -> Result<Duration> {
        self.command_response_duration("superkeys.timeout", TimeUnit::Milliseconds)
    }

    /// Sets the Superkeys timeout of how long it waits for the next tap.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeystimeout
    pub fn superkeys_timeout_set(&mut self, duration: Duration) -> Result<()> {
        if self.superkeys_timeout_get()? == duration {
            return Ok(());
        }

        self.command(&format!("superkeys.timeout {}", &duration.as_millis()))
    }

    /// Gets the Superkeys repeat duration.
    ///
    /// The repeat value specifies the time between the second and subsequent key code releases when on hold, it only takes effect after the wait for timer has been exceeded.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeysrepeat
    pub fn superkeys_repeat_get(&mut self) -> Result<Duration> {
        self.command_response_duration("superkeys.repeat", TimeUnit::Milliseconds)
    }

    /// Sets the Superkeys repeat duration.
    ///
    /// The repeat value specifies the time between the second and subsequent key code releases when on hold, it only takes effect after the wait for timer has been exceeded.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeysrepeat
    pub fn superkeys_repeat_set(&mut self, duration: Duration) -> Result<()> {
        if self.superkeys_repeat_get()? == duration {
            return Ok(());
        }

        self.command(&format!("superkeys.repeat {}", &duration.as_millis()))
    }

    /// Gets the Superkeys hold start duration.
    ///
    /// The hold start value specifies the minimum time that has to pass between the first key down and any other action to trigger a hold, if held it will emit a hold action.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeysholdstart
    pub fn superkeys_hold_start_get(&mut self) -> Result<Duration> {
        self.command_response_duration("superkeys.holdstart", TimeUnit::Milliseconds)
    }

    /// Sets the Superkeys hold start duration.
    ///
    /// The hold start value specifies the minimum time that has to pass between the first key down and any other action to trigger a hold, if held it will emit a hold action.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeysholdstart
    pub fn superkeys_hold_start_set(&mut self, duration: Duration) -> Result<()> {
        if self.superkeys_hold_start_get()? == duration {
            return Ok(());
        }

        self.command(&format!("superkeys.holdstart {}", &duration.as_millis()))
    }

    /// Gets the Superkeys overlap percentage.
    ///
    /// The overlap value specifies the percentage of overlap when fast typing that is allowed to happen before triggering a hold action to the overlapped key pressed after the super key.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeysoverlap
    pub fn superkeys_overlap_get(&mut self) -> Result<u8> {
        self.command_response_numerical("superkeys.overlap")
    }

    /// Sets the Superkeys overlap percentage.
    ///
    /// The overlap value specifies the percentage of overlap when fast typing that is allowed to happen before triggering a hold action to the overlapped key pressed after the super key.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#superkeysoverlap
    pub fn superkeys_overlap_set(&mut self, percentage: u8) -> Result<()> {
        if percentage > 80 {
            bail!("Percentage must be 80 or below: {}", percentage);
        }

        if self.superkeys_overlap_get()? == percentage {
            return Ok(());
        }

        self.command(&format!("superkeys.overlap {}", percentage))
    }

    /// Gets the color of a specific LED.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledat
    pub fn led_at_get(&mut self, led: u8) -> Result<Color> {
        let response = self.command_response_string(&format!("led.at {}", led))?;

        if response.is_empty() {
            bail!("Empty response");
        }

        let parts = response.split_whitespace().collect::<Vec<&str>>();

        if parts.len() != 3 {
            bail!("Response does not contain exactly three parts");
        }

        let r = parts[0].parse()?;
        let g = parts[1].parse()?;
        let b = parts[2].parse()?;

        Ok(Color { r, g, b })
    }

    /// Sets the color of a specific LED.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledat
    pub fn led_at_set(&mut self, led: u8, color: &Color) -> Result<()> {
        if &self.led_at_get(led)? == color {
            return Ok(());
        }

        self.command(&format!(
            "led.at {} {} {} {}",
            led, color.r, color.g, color.b
        ))
    }

    /// Sets the color of all the LEDs.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledsetall
    pub fn led_all_set(&mut self, color: &Color) -> Result<()> {
        self.command(&format!("led.setAll {} {} {}", color.r, color.g, color.b,))
    }

    /// Gets the LED mode.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledmode
    pub fn led_mode_get(&mut self) -> Result<LedMode> {
        self.command_response_numerical("led.mode")
    }

    /// Sets the LED mode.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledmode
    pub fn led_mode_set(&mut self, mode: LedMode) -> Result<()> {
        if self.led_mode_get()? == mode {
            return Ok(());
        }

        self.command(&format!("led.mode {}", &mode.value()))
    }

    /// Gets the LED brightness.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledbrightness
    pub fn led_brightness_get(&mut self) -> Result<u8> {
        self.command_response_numerical("led.brightness")
    }

    /// Sets the LED brightness.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledbrightness
    pub fn led_brightness_set(&mut self, brightness: u8) -> Result<()> {
        if self.led_brightness_get()? == brightness {
            return Ok(());
        }

        self.command(&format!("led.brightness {}", brightness))
    }

    /// Gets the underglow LED brightness.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledbrightnessug
    pub fn led_brightness_underglow_get(&mut self) -> Result<u8> {
        self.command_response_numerical("led.brightnessUG")
    }

    /// Sets the underglow LED brightness.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledbrightnessug
    pub fn led_brightness_underglow_set(&mut self, brightness: u8) -> Result<()> {
        if self.led_brightness_underglow_get()? == brightness {
            return Ok(());
        }

        self.command(&format!("led.brightnessUG {}", brightness))
    }

    /// Gets the wireless LED brightness.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledbrightness
    pub fn led_brightness_wireless_get(&mut self) -> Result<u8> {
        self.command_response_numerical("led.brightness.wireless")
    }

    /// Sets the wireless LED brightness.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledbrightness
    pub fn led_brightness_wireless_set(&mut self, brightness: u8) -> Result<()> {
        if self.led_brightness_wireless_get()? == brightness {
            return Ok(());
        }

        self.command(&format!("led.brightness.wireless {}", brightness))
    }

    /// Gets the wireless underglow LED brightness.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledbrightnessug
    pub fn led_brightness_underglow_wireless_get(&mut self) -> Result<u8> {
        self.command_response_numerical("led.brightnessUG.wireless")
    }

    /// Sets the wireless underglow LED brightness.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledbrightnessug
    pub fn led_brightness_underglow_wireless_set(&mut self, brightness: u8) -> Result<()> {
        if self.led_brightness_underglow_wireless_get()? == brightness {
            return Ok(());
        }

        self.command(&format!("led.brightnessUG.wireless {}", brightness))
    }

    /// Gets the LED fade.
    pub fn led_fade_get(&mut self) -> Result<u16> {
        self.command_response_numerical("led.fade")
    }

    /// Sets the LED fade.
    pub fn led_fade_set(&mut self, fade: u16) -> Result<()> {
        if self.led_fade_get()? == fade {
            return Ok(());
        }

        self.command(&format!("led.fade {}", fade))
    }

    /// Gets the LED theme.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledtheme
    pub fn led_theme_get(&mut self) -> Result<String> {
        self.command_response_string("led.theme")
    }

    /// Sets the LED theme.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#ledtheme
    pub fn led_theme_set(&mut self, data: &str) -> Result<()> {
        if self.led_theme_get()? == data {
            return Ok(());
        }

        self.command(&format!("led.theme {}", data))
    }

    /// Gets the palette.
    ///
    /// The color palette is used by the color map to establish each color that can be assigned to the keyboard.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#palette
    pub fn palette_get(&mut self) -> Result<String> {
        self.command_response_string("palette")
    }

    /// Sets the palette.
    ///
    /// The color palette is used by the color map to establish each color that can be assigned to the keyboard.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#palette
    pub fn palette_set(&mut self, data: &str) -> Result<()> {
        if self.palette_get()? == data {
            return Ok(());
        }

        self.command(&format!("palette {}", data))
    }

    /// Gets the color map.
    ///
    /// This command reads the color map that assigns each color listed in the palette to individual LEDs, mapping them to the keyboard's current layout.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#colormapmap
    pub fn color_map_get(&mut self) -> Result<String> {
        self.command_response_string("colormap.map")
    }

    /// Sets the color map.
    ///
    /// This command writes the color map that assigns each color listed in the palette to individual LEDs, mapping them to the keyboard's current layout.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#colormapmap
    pub fn color_map_set(&mut self, data: &str) -> Result<()> {
        if self.color_map_get()? == data {
            return Ok(());
        }

        self.command(&format!("colormap.map {}", data))
    }

    /// Gets the idle LED true sleep state.
    pub fn led_idle_true_sleep_get(&mut self) -> Result<bool> {
        self.command_response_bool("idleleds.true_sleep")
    }

    /// Sets the idle LED true sleep state.
    pub fn led_idle_true_sleep_set(&mut self, state: bool) -> Result<()> {
        if self.led_idle_true_sleep_get()? == state {
            return Ok(());
        }

        self.command(&format!("idleleds.true_sleep {}", state as u8))
    }

    /// Gets the idle LED true sleep time.
    pub fn led_idle_true_sleep_time_get(&mut self) -> Result<Duration> {
        self.command_response_duration("idleleds.true_sleep_time", TimeUnit::Seconds)
    }

    /// Sets the idle LED true sleep time.
    pub fn led_idle_true_sleep_time_set(&mut self, duration: Duration) -> Result<()> {
        let seconds = duration.as_secs();

        if seconds > 65_000 {
            bail!("Seconds must be 65000 or below: {}", seconds);
        }

        if self.led_idle_true_sleep_time_get()? == duration {
            return Ok(());
        }

        self.command(&format!("idleleds.true_sleep_time {}", seconds))
    }

    /// Gets the idle LED time limit.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#idleledstime_limit
    pub fn led_idle_time_limit_get(&mut self) -> Result<Duration> {
        self.command_response_duration("idleleds.time_limit", TimeUnit::Seconds)
    }

    /// Sets the idle LED time limit.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#idleledstime_limit
    pub fn led_idle_time_limit_set(&mut self, duration: Duration) -> Result<()> {
        let seconds = duration.as_secs();

        if seconds > 65_000 {
            bail!("Duration must be 65000 seconds or below, got: {}", seconds);
        }

        if self.led_idle_time_limit_get()? == duration {
            return Ok(());
        }

        self.command(&format!("idleleds.time_limit {}", seconds))
    }

    /// Gets the idle LED wireless state.
    pub fn led_idle_wireless_get(&mut self) -> Result<bool> {
        self.command_response_bool("idleleds.wireless")
    }

    /// Sets the idle LED wireless state.
    pub fn led_idle_wireless_set(&mut self, state: bool) -> Result<()> {
        if self.led_idle_wireless_get()? == state {
            return Ok(());
        }

        self.command(&format!("idleleds.wireless {}", state as u8))
    }

    /// Gets the keyboard model name.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#hardwareversion
    pub fn hardware_version_get(&mut self) -> Result<String> {
        self.command_response_string("hardware.version")
    }

    /// Sets the keyboard model name.
    pub fn hardware_version_set(&mut self, data: &str) -> Result<()> {
        if self.hardware_version_get()? == data {
            return Ok(());
        }

        self.command(&format!("hardware.version {}", data))
    }

    // TODO: hardware.side_power https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#hardwareside_power
    // TODO: hardware.side_ver https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#hardwareside_ver
    // TODO: hardware.keyscanInterval
    // TODO: hardware.firmware https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#hardwarefirmware
    // TODO: hardware.chip_id
    // TODO: hardware.chip_info

    /// Gets the Qukeys hold timeout.
    ///
    /// https://kaleidoscope.readthedocs.io/en/latest/plugins/Kaleidoscope-Qukeys.html
    pub fn qukeys_hold_timeout_get(&mut self) -> Result<Duration> {
        self.command_response_duration("qukeys.holdTimeout", TimeUnit::Milliseconds)
    }

    /// Sets the Qukeys hold timeout.
    ///
    /// https://kaleidoscope.readthedocs.io/en/latest/plugins/Kaleidoscope-Qukeys.html
    pub fn qukeys_hold_timeout_set(&mut self, duration: Duration) -> Result<()> {
        if self.qukeys_hold_timeout_get()? == duration {
            return Ok(());
        }

        self.command(&format!("qukeys.holdTimeout {}", &duration.as_millis()))
    }

    /// Gets the Qukeys overlap threshold.
    ///
    /// https://kaleidoscope.readthedocs.io/en/latest/plugins/Kaleidoscope-Qukeys.html
    pub fn qukeys_overlap_threshold_get(&mut self) -> Result<Duration> {
        self.command_response_duration("qukeys.overlapThreshold", TimeUnit::Milliseconds)
    }

    /// Sets the Qukeys overlap threshold.
    ///
    /// https://kaleidoscope.readthedocs.io/en/latest/plugins/Kaleidoscope-Qukeys.html
    pub fn qukeys_overlap_threshold_set(&mut self, duration: Duration) -> Result<()> {
        if self.qukeys_overlap_threshold_get()? == duration {
            return Ok(());
        }

        self.command(&format!(
            "qukeys.overlapThreshold {}",
            &duration.as_millis()
        ))
    }

    /// Gets the macros map.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#macrosmap
    pub fn macros_map_get(&mut self) -> Result<String> {
        self.command_response_string("macros.map")
    }

    /// Sets the macros map.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#macrosmap
    pub fn macros_map_set(&mut self, data: &str) -> Result<()> {
        if self.macros_map_get()? == data {
            return Ok(());
        }

        self.command(&format!("macros.map {}", data))
    }

    /// Triggers a macro.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#macrostrigger
    pub fn macros_trigger(&mut self, macro_id: u8) -> Result<()> {
        self.command(&format!("macros.trigger {}", macro_id))
    }

    /// Gets the macros memory size in bytes.
    pub fn macros_memory_get(&mut self) -> Result<u16> {
        self.command_response_numerical("macros.memory")
    }

    /// Gets all the available commands in the current version of the serial protocol.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#help
    pub fn help_get(&mut self) -> Result<Vec<String>> {
        self.command_response_vec_string("help")
    }

    /// Gets the virtual mouse speed.
    pub fn mouse_speed_get(&mut self) -> Result<u8> {
        self.command_response_numerical("mouse.speed")
    }

    /// Sets the virtual mouse speed.
    pub fn mouse_speed_set(&mut self, speed: u8) -> Result<()> {
        if speed > 127 {
            bail!("Speed out of range, max is {}: {}", 127, speed);
        }

        if self.mouse_speed_get()? == speed {
            return Ok(());
        }

        self.command(&format!("mouse.speed {}", speed))
    }

    /// Gets the virtual mouse delay.
    pub fn mouse_delay_get(&mut self) -> Result<Duration> {
        self.command_response_duration("mouse.speedDelay", TimeUnit::Milliseconds)
    }

    /// Sets the virtual mouse delay.
    pub fn mouse_delay_set(&mut self, duration: Duration) -> Result<()> {
        if self.mouse_delay_get()? == duration {
            return Ok(());
        }

        self.command(&format!("mouse.speedDelay {}", &duration.as_millis()))
    }

    /// Gets the virtual mouse acceleration speed.
    pub fn mouse_acceleration_speed_get(&mut self) -> Result<u8> {
        self.command_response_numerical("mouse.accelSpeed")
    }

    /// Sets the virtual mouse acceleration speed.
    pub fn mouse_acceleration_speed_set(&mut self, speed: u8) -> Result<()> {
        if self.mouse_acceleration_speed_get()? == speed {
            return Ok(());
        }

        self.command(&format!("mouse.accelSpeed {}", speed))
    }

    /// Gets the virtual mouse acceleration delay.
    pub fn mouse_acceleration_delay_get(&mut self) -> Result<Duration> {
        self.command_response_duration("mouse.accelDelay", TimeUnit::Milliseconds)
    }

    /// Sets the virtual mouse acceleration delay.
    pub fn mouse_acceleration_delay_set(&mut self, duration: Duration) -> Result<()> {
        if self.mouse_acceleration_delay_get()? == duration {
            return Ok(());
        }

        self.command(&format!("mouse.accelDelay {}", &duration.as_millis()))
    }

    /// Gets the virtual mouse wheel speed.
    pub fn mouse_wheel_speed_get(&mut self) -> Result<u8> {
        self.command_response_numerical("mouse.wheelSpeed")
    }

    /// Sets the virtual mouse wheel speed.
    pub fn mouse_wheel_speed_set(&mut self, speed: u8) -> Result<()> {
        if self.mouse_wheel_speed_get()? == speed {
            return Ok(());
        }

        self.command(&format!("mouse.wheelSpeed {}", speed))
    }

    /// Gets the virtual mouse wheel delay.
    pub fn mouse_wheel_delay_get(&mut self) -> Result<Duration> {
        self.command_response_duration("mouse.wheelDelay", TimeUnit::Milliseconds)
    }

    /// Sets the virtual mouse wheel delay.
    pub fn mouse_wheel_delay_set(&mut self, duration: Duration) -> Result<()> {
        if self.mouse_wheel_delay_get()? == duration {
            return Ok(());
        }

        self.command(&format!("mouse.wheelDelay {}", &duration.as_millis()))
    }

    /// Gets the virtual mouse speed limit.
    pub fn mouse_speed_limit_get(&mut self) -> Result<u8> {
        self.command_response_numerical("mouse.speedLimit")
    }

    /// Sets the virtual mouse speed limit.
    pub fn mouse_speed_limit_set(&mut self, limit: u8) -> Result<()> {
        if self.mouse_speed_limit_get()? == limit {
            return Ok(());
        }

        self.command(&format!("mouse.speedLimit {}", limit))
    }

    /// Activate a certain layer remotely just by sending its order number.
    ///
    /// The layer is -1 to Bazecor.
    ///
    /// This does not affect the memory usage as the value is stored in RAM.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#layeractivate
    pub fn layer_activate(&mut self, layer: u8) -> Result<()> {
        self.command(&format!("layer.activate {}", layer))
    }

    /// Deactivate the last layer that the keyboard switched to.
    /// This same function is the way the shift to layer key works on the keyboard.
    ///
    /// Just provide the layer number to make the keyboard go back one layer. The layer is -1 to Bazecor.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#layerdeactivate
    pub fn layer_deactivate(&mut self, layer: Option<u8>) -> Result<()> {
        if let Some(layer) = layer {
            if layer > MAX_LAYERS {
                bail!("Layer out of range, max is {}: {}", MAX_LAYERS, layer);
            }
            self.command(&format!("layer.deactivate {}", layer))?
        }

        self.command("layer.deactivate")
    }

    /// Gets the state of the provided layer.
    ///
    /// The layer is -1 to Bazecor.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#layerisactive
    pub fn layer_is_active_get(&mut self, layer: u8) -> Result<bool> {
        if layer > MAX_LAYERS {
            bail!("Layer out of range, max is {}: {}", MAX_LAYERS, layer);
        }
        self.command_response_bool(&format!("layer.isActive {}", layer))
    }

    /// Switch to a certain layer.
    ///
    /// The layer is -1 to Bazecor.
    ///
    /// The difference between this command and the layer_activate alternative, is that the layer_activate adds to the layer switching history, but moveTo will erase that memory and return it to an array length 1 and holding the current layer the keyboard moved to.
    ///
    /// This does not affect the memory usage as the value is stored in RAM.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#layermoveto
    pub fn layer_move_to(&mut self, layer: u8) -> Result<()> {
        self.command(&format!("layer.moveTo {}", layer))
    }

    /// Gets the status for up to 32 layers.
    ///
    /// It will return a vector of bools with the respective index matching each layer, -1 from Bazecor.
    ///
    /// https://github.com/Dygmalab/Bazecor/blob/development/FOCUS_API.md#layerstate
    pub fn layer_state_get(&mut self) -> Result<Vec<bool>> {
        let response = self.command_response_string("layer.state")?;
        let parts = response.split_whitespace().collect::<Vec<&str>>();
        let nums = parts.iter().map(|&part| part == "1").collect();

        Ok(nums)
    }

    /// Gets the battery level of the left keyboard as a percentage.
    pub fn wireless_battery_level_left_get(&mut self) -> Result<u8> {
        self.command_response_numerical("wireless.battery.left.level")
    }

    /// Gets the battery level of the right keyboard as a percentage.
    pub fn wireless_battery_level_right_get(&mut self) -> Result<u8> {
        self.command_response_numerical("wireless.battery.right.level")
    }

    /// Gets the battery status of the left keyboard.
    pub fn wireless_battery_status_left_get(&mut self) -> Result<u8> {
        self.command_response_numerical("wireless.battery.left.status")
    }

    /// Gets the battery status of the right keyboard.
    pub fn wireless_battery_status_right_get(&mut self) -> Result<u8> {
        self.command_response_numerical("wireless.battery.right.status")
    }

    /// Gets the battery saving mode state.
    pub fn wireless_battery_saving_mode_get(&mut self) -> Result<bool> {
        self.command_response_bool("wireless.battery.savingMode")
    }

    /// Sets the battery saving mode state.
    pub fn wireless_battery_saving_mode_set(&mut self, state: bool) -> Result<()> {
        if self.wireless_battery_saving_mode_get()? == state {
            return Ok(());
        }

        self.command(&format!("wireless.battery.savingMode {}", state as u8))
    }

    /// Gets the RF power level.
    pub fn wireless_rf_power_level_get(&mut self) -> Result<WirelessPowerMode> {
        self.command_response_numerical("wireless.rf.power")
    }

    /// Sets the RF power level.
    pub fn wireless_rf_power_level_set(
        &mut self,
        wireless_power_mode: WirelessPowerMode,
    ) -> Result<()> {
        if self.wireless_rf_power_level_get()? == wireless_power_mode {
            return Ok(());
        }

        self.command(&format!(
            "wireless.rf.power {}",
            &wireless_power_mode.value()
        ))
    }

    /// Gets the RF channel hop state.
    pub fn wireless_rf_channel_hop_get(&mut self) -> Result<bool> {
        self.command_response_bool("wireless.rf.channelHop")
    }

    /// Sets the RF channel hop state.
    pub fn wireless_rf_channel_hop_set(&mut self, state: bool) -> Result<()> {
        if self.wireless_rf_channel_hop_get()? == state {
            return Ok(());
        }

        self.command(&format!("wireless.rf.channelHop {}", state as u8))
    }

    /// Gets the sync pairing state.
    pub fn wireless_rf_sync_pairing_get(&mut self) -> Result<bool> {
        self.command_response_bool("wireless.rf.syncPairing")
    }
}

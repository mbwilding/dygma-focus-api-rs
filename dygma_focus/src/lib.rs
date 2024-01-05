use crate::keyboards::Keyboard;
use anyhow::{anyhow, Result};
use log::error;

pub mod api;
pub mod color;
pub mod enums;
pub mod keyboards;
pub mod prelude;

pub const MAX_LAYERS: u8 = 10 - 1;

/// The Dygma Focus API.
#[cfg(not(target_arch = "wasm32"))]
pub struct Focus {
    pub(crate) serial: Box<dyn serialport::SerialPort>,
    pub(crate) response_buffer: Vec<u8>,
}

/// The Dygma Focus API.
#[cfg(target_arch = "wasm32")]
pub struct Focus {
    pub(crate) serial: web_sys::SerialPort,
    pub(crate) response_buffer: Vec<u8>,
}

/// Constructors
impl Focus {
    /// Creates a new instance of the Focus API, connecting to the keyboard via port.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_via_port(port: &str) -> Result<Self> {
        let port_settings = serialport::new(port, 115_200)
            .data_bits(serialport::DataBits::Eight)
            .flow_control(serialport::FlowControl::None)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .timeout(std::time::Duration::from_millis(40));

        let mut serial = port_settings.open().map_err(|e| {
            let err_msg = format!("Failed to open serial port: {} ({:?})", &port, e);
            error!("{}", err_msg);
            anyhow!(err_msg)
        })?;

        serial.write_data_terminal_ready(true)?;

        Ok(Self {
            serial,
            response_buffer: Vec::with_capacity(4096),
        })
    }

    /// Creates a new instance of the Focus API, connecting to the keyboard via port.
    #[cfg(target_arch = "wasm32")]
    pub fn new_via_port(port: &str) -> Result<Self> {
        let serial = web_sys::SerialPort::new_with_usb_serial_options_and_event_target(
            port,
            web_sys::SerialOptions::new()
                .baud_rate(115_200)
                .data_bits(web_sys::SerialDataBits::Eight)
                .flow_control(web_sys::SerialFlowControl::None)
                .parity(web_sys::SerialParity::None)
                .stop_bits(web_sys::SerialStopBits::One),
            &web_sys::window().unwrap(),
        );

        Ok(Self {
            serial,
            response_buffer: vec![],
        })
    }

    /// Creates a new instance of the Focus API, connecting to the keyboard via keyboard struct.
    pub fn new_via_keyboard(device: &Keyboard) -> Result<Self> {
        Self::new_via_port(&device.port)
    }

    /// Creates a new instance of the Focus API, connecting to the keyboard via first available keyboard.
    pub fn new_first_available() -> Result<Self> {
        Self::new_via_keyboard(Keyboard::find_all_keyboards()?.first().ok_or_else(|| {
            let err_msg = "No supported keyboards found";
            error!("{}", err_msg);
            anyhow!(err_msg)
        })?)
    }
}

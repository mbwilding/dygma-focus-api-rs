use crate::hardware::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEVICES_PHYSICAL: [Hardware; 8] = [
        DEFY_WIRED,
        DEFY_WIRED_BOOTLOADER,
        DEFY_WIRELESS,
        DEFY_WIRELESS_BOOTLOADER,
        RAISE_ANSI,
        RAISE_ANSI_BOOTLOADER,
        RAISE_ISO,
        RAISE_ISO_BOOTLOADER,
    ];
}

pub const DEFY_WIRED: Hardware = {
    Hardware {
        info: Info {
            vendor: Vendor::Dygma,
            product: Product::Defy,
            device_type: DeviceType::Wired,
            display_name: "Dygma Defy Wired",
            urls: Urls {
                homepage: Url {
                    name: "Homepage",
                    url: "https://www.dygma.com/defy/",
                },
            },
        },
        usb: Usb {
            vendor_id: 0x35ef,
            product_id: 0x0010,
        },
        bootloader: false,
        keyboard: Some(Grid {
            rows: 5,
            columns: 16,
        }),
        keyboard_underglow: Some(Grid {
            rows: 2,
            columns: 89,
        }),
        rgbw_mode: true,
        instructions: Languages {
            en: Dialog {
                update_instructions: "To update the firmware, the keyboard needs a special reset. When the countdown starts, press and hold the Escape key. Soon after the countdown finished, the Neuron's light should start a blue pulsing pattern, and the flashing will proceed. At this point, you should release the Escape key.",
            },
        },
        virtual_info: None,
    }
};

pub const DEFY_WIRED_BOOTLOADER: Hardware = {
    Hardware {
        info: Info {
            vendor: Vendor::Dygma,
            product: Product::Defy,
            device_type: DeviceType::Wired,
            display_name: "Dygma Defy Wired (Bootloader)",
            urls: Urls {
                homepage: Url {
                    name: "Homepage",
                    url: "https://www.dygma.com/defy/",
                },
            },
        },
        usb: Usb {
            vendor_id: 0x35ef,
            product_id: 0x0011,
        },
        bootloader: true,
        keyboard: None,
        keyboard_underglow: None,
        rgbw_mode: true,
        instructions: Languages {
            en: Dialog {
                update_instructions: "To update the firmware, press the button at the bottom. You must not hold any key on the keyboard while the countdown is in progress, nor afterwards, until the flashing is finished. When the countdown reaches zero, the Neuron's light should start a blue pulsing pattern, and flashing will then proceed.",
            },
        },
        virtual_info: None,
    }
};

pub const DEFY_WIRELESS: Hardware = {
    Hardware {
        info: Info {
            vendor: Vendor::Dygma,
            product: Product::Defy,
            device_type: DeviceType::Wireless,
            display_name: "Dygma Defy Wireless",
            urls: Urls {
                homepage: Url {
                    name: "Homepage",
                    url: "https://www.dygma.com/defy/",
                },
            },
        },
        usb: Usb {
            vendor_id: 0x35ef,
            product_id: 0x0012,
        },
        bootloader: false,
        keyboard: Some(Grid {
            rows: 5,
            columns: 16,
        }),
        keyboard_underglow: Some(Grid {
            rows: 2,
            columns: 89,
        }),
        rgbw_mode: true,
        instructions: Languages {
            en: Dialog {
                update_instructions: "To update the firmware, the keyboard needs a special reset. When the countdown starts, press and hold the Escape key. Soon after the countdown finished, the Neuron's light should start a blue pulsing pattern, and the flashing will proceed. At this point, you should release the Escape key.",
            },
        },
        virtual_info: None,
    }
};

pub const DEFY_WIRELESS_BOOTLOADER: Hardware = {
    Hardware {
        info: Info {
            vendor: Vendor::Dygma,
            product: Product::Defy,
            device_type: DeviceType::Wireless,
            display_name: "Dygma Defy Wireless (Bootloader)",
            urls: Urls {
                homepage: Url {
                    name: "Homepage",
                    url: "https://www.dygma.com/defy/",
                },
            },
        },
        usb: Usb {
            vendor_id: 0x35ef,
            product_id: 0x0013,
        },
        bootloader: true,
        keyboard: None,
        keyboard_underglow: None,
        rgbw_mode: true,
        instructions: Languages {
            en: Dialog {
                update_instructions: "To update the firmware, press the button at the bottom. You must not hold any key on the keyboard while the countdown is in progress, nor afterwards, until the flashing is finished. When the countdown reaches zero, the Neuron's light should start a blue pulsing pattern, and flashing will then proceed.",
            },
        },
        virtual_info: None,
    }
};

pub const RAISE_ANSI: Hardware = {
    Hardware {
        info: Info {
            vendor: Vendor::Dygma,
            product: Product::Raise,
            device_type: DeviceType::ANSI,
            display_name: "Dygma Raise ANSI",
            urls: Urls {
                homepage: Url {
                    name: "Homepage",
                    url: "https://www.dygma.com/raise/",
                },
            },
        },
        usb: Usb {
            vendor_id: 0x1209,
            product_id: 0x2201,
        },
        bootloader: false,
        keyboard: Some(Grid {
            rows: 5,
            columns: 16,
        }),
        keyboard_underglow: Some(Grid {
            rows: 6,
            columns: 22,
        }),
        rgbw_mode: false,
        instructions: Languages {
            en: Dialog {
                update_instructions: "To update the firmware, the keyboard needs a special reset. When the countdown starts, press and hold the Escape key. Soon after the countdown finished, the Neuron's light should start a blue pulsing pattern, and the flashing will proceed. At this point, you should release the Escape key.",
            },
        },
        virtual_info: None,
    }
};

pub const RAISE_ANSI_BOOTLOADER: Hardware = {
    Hardware {
        info: Info {
            vendor: Vendor::Dygma,
            product: Product::Raise,
            device_type: DeviceType::ANSI,
            display_name: "Dygma Raise ANSI (Bootloader)",
            urls: Urls {
                homepage: Url {
                    name: "Homepage",
                    url: "https://www.dygma.com/raise/",
                },
            },
        },
        usb: Usb {
            vendor_id: 0x1209,
            product_id: 0x2200,
        },
        bootloader: true,
        keyboard: None,
        keyboard_underglow: None,
        rgbw_mode: false,
        instructions: Languages {
            en: Dialog {
                update_instructions: "To update the firmware, press the button at the bottom. You must not hold any key on the keyboard while the countdown is in progress, nor afterwards, until the flashing is finished. When the countdown reaches zero, the Neuron's light should start a blue pulsing pattern, and flashing will then proceed.",
            },
        },
        virtual_info: None,
    }
};

pub const RAISE_ISO: Hardware = {
    Hardware {
        info: Info {
            vendor: Vendor::Dygma,
            product: Product::Raise,
            device_type: DeviceType::ISO,
            display_name: "Dygma Raise ISO",
            urls: Urls {
                homepage: Url {
                    name: "Homepage",
                    url: "https://www.dygma.com/raise/",
                },
            },
        },
        usb: Usb {
            vendor_id: 0x1209,
            product_id: 0x2201,
        },
        bootloader: false,
        keyboard: Some(Grid {
            rows: 5,
            columns: 16,
        }),
        keyboard_underglow: Some(Grid {
            rows: 6,
            columns: 22,
        }),
        rgbw_mode: false,
        instructions: Languages {
            en: Dialog {
                update_instructions: "To update the firmware, the keyboard needs a special reset. When the countdown starts, press and hold the Escape key. Soon after the countdown finished, the Neuron's light should start a blue pulsing pattern, and the flashing will proceed. At this point, you should release the Escape key.",
            },
        },
        virtual_info: None,
    }
};

pub const RAISE_ISO_BOOTLOADER: Hardware = {
    Hardware {
        info: Info {
            vendor: Vendor::Dygma,
            product: Product::Raise,
            device_type: DeviceType::ISO,
            display_name: "Dygma Raise ISO (Bootloader)",
            urls: Urls {
                homepage: Url {
                    name: "Homepage",
                    url: "https://www.dygma.com/raise/",
                },
            },
        },
        usb: Usb {
            vendor_id: 0x1209,
            product_id: 0x2200,
        },
        bootloader: true,
        keyboard: None,
        keyboard_underglow: None,
        rgbw_mode: false,
        instructions: Languages {
            en: Dialog {
                update_instructions: "To update the firmware, press the button at the bottom. You must not hold any key on the keyboard while the countdown is in progress, nor afterwards, until the flashing is finished. When the countdown reaches zero, the Neuron's light should start a blue pulsing pattern, and flashing will then proceed.",
            },
        },
        virtual_info: None,
    }
};

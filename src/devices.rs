use anyhow::{bail, Result};

pub struct DevicePreset {
    pub name: &'static str,
    pub width: u32,
    pub height: u32,
    pub user_agent: &'static str,
    pub scale: f64,
    pub is_mobile: bool,
    pub has_touch: bool,
}

static DEVICES: &[DevicePreset] = &[
    DevicePreset {
        name: "iphone-15",
        width: 393,
        height: 852,
        scale: 3.0,
        is_mobile: true,
        has_touch: true,
        user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    },
    DevicePreset {
        name: "iphone-15-pro-max",
        width: 430,
        height: 932,
        scale: 3.0,
        is_mobile: true,
        has_touch: true,
        user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    },
    DevicePreset {
        name: "iphone-se",
        width: 375,
        height: 667,
        scale: 2.0,
        is_mobile: true,
        has_touch: true,
        user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    },
    DevicePreset {
        name: "pixel-7",
        width: 412,
        height: 915,
        scale: 2.625,
        is_mobile: true,
        has_touch: true,
        user_agent: "Mozilla/5.0 (Linux; Android 14; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    },
    DevicePreset {
        name: "pixel-8",
        width: 412,
        height: 932,
        scale: 2.625,
        is_mobile: true,
        has_touch: true,
        user_agent: "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    },
    DevicePreset {
        name: "galaxy-s24",
        width: 360,
        height: 780,
        scale: 3.0,
        is_mobile: true,
        has_touch: true,
        user_agent: "Mozilla/5.0 (Linux; Android 14; SM-S921B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36",
    },
    DevicePreset {
        name: "ipad-pro",
        width: 1024,
        height: 1366,
        scale: 2.0,
        is_mobile: true,
        has_touch: true,
        user_agent: "Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    },
    DevicePreset {
        name: "ipad-air",
        width: 820,
        height: 1180,
        scale: 2.0,
        is_mobile: true,
        has_touch: true,
        user_agent: "Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    },
    DevicePreset {
        name: "macbook-pro-14",
        width: 1512,
        height: 982,
        scale: 2.0,
        is_mobile: false,
        has_touch: false,
        user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
    },
    DevicePreset {
        name: "macbook-air-13",
        width: 1470,
        height: 956,
        scale: 2.0,
        is_mobile: false,
        has_touch: false,
        user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
    },
];

pub fn find_device(name: &str) -> Result<&'static DevicePreset> {
    let normalized = name.to_lowercase().replace(' ', "-");

    for device in DEVICES {
        if device.name == normalized {
            return Ok(device);
        }
    }

    let available: Vec<&str> = DEVICES.iter().map(|d| d.name).collect();
    bail!(
        "unknown device '{}'. Available devices: {}",
        name,
        available.join(", ")
    );
}

use hidapi::HidDevice;

pub struct M5 {
    device: HidDevice,
}

impl M5 {
    pub fn new(vid: u16, pid: u16) -> Self {
        let api = hidapi::HidApi::new().unwrap();

        let devices = api.device_list();

        let device = devices
            .into_iter()
            .find(|x| x.vendor_id() == vid && x.product_id() == pid && x.usage_page() == 0xff01)
            .unwrap();

        M5 {
            device: api.open_path(device.path()).unwrap(),
        }
    }

    pub fn set_lighting(&self, mode: u8, brightness: u8, color: u32) {
        let red = ((color >> 16) & 0xFF) as u8;
        let green = ((color >> 8) & 0xFF) as u8;
        let blue = (color & 0xFF) as u8;
        let buf = [0x51, 0x28, 0x03, 0x00, mode, brightness, red, green, blue];

        self.device.write(&buf).unwrap();
        println!("Lighting settings updated.");
    }

    pub fn set_profile(&self, profile: u8) {
        let buf = [0x50, 0x02, profile - 1];
        self.device.write(&buf).unwrap();
        println!("Profile {profile} active.")
    }

    pub fn set_dpi1(&self, dpi: u16) {
        let value = ((dpi / 100) - 1) as u8;
        let buf = [0x51, 0x31, 0x00, 0x00, value];
        self.device.write(&buf).unwrap();
        println!("Updated primary dpi to {dpi}.")
    }

    pub fn set_dpi2(&self, dpi: u16) {
        let value = ((dpi / 100) - 1) as u8;
        let buf = [0x51, 0x31, 0x01, 0x00, value];
        self.device.write(&buf).unwrap();
        println!("Updated secondary dpi to {dpi}.")
    }

    pub fn set_polling_rate(&self, rate: u16) {
        let value: u8 = match rate {
            n if n <= 125 => 0x00,
            n if n <= 250 => 0x01,
            n if n <= 500 => 0x02,
            _ => 0x03,
        };

        let buf = [0x51, 0x31, 0x02, 0x00, value];
        self.device.write(&buf).unwrap();
        println!("Button polling rate updated.")
    }

    pub fn set_button_response(&self, ms: u8) {
        let value = ms / 4;
        let buf = [0x51, 0x31, 0x03, 0x00, value];
        self.device.write(&buf).unwrap();
        println!("Button response time updated.")
    }

    pub fn set_angle_snapping(&self, active: bool) {
        let value: u8 = if active { 0x00 } else { 0x01 };
        let buf = [0x51, 0x31, 0x03, 0x00, value];
        self.device.write(&buf).unwrap();
        println!(
            "Turned {} angle snapping.",
            if active { "on" } else { "off" }
        )
    }
}

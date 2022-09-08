use ia_devices::device::TDevice;
use std::fmt;

use crate::homeerrors::HomeErrors;

pub struct Room {
    pub name: String,
    devices: Vec<Box<dyn TDevice + Send>>,
}

impl Room {
    fn get_device_pos(&self, device_name: &str) -> Result<usize, HomeErrors> {
        let pos = self
            .devices
            .iter()
            .position(|x| -> bool { *x.get_name() == *device_name });
        if let Some(x) = pos {
            return Ok(x);
        }
        Err(HomeErrors::NotExist(device_name.to_string()))
    }

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: Vec::new(),
        }
    }
    pub fn add_device(&mut self, device: impl TDevice) -> Result<(), HomeErrors> {
        let device_name = device.get_name();
        let pos = self.get_device_pos(device_name);
        if let Ok(_) = pos {
            return Err(HomeErrors::AlreadyExist(device_name.clone()));
        }
        let tmp = device.get_box();
        self.devices.push(tmp);
        Ok(())
    }
    pub fn remove_device(&mut self, device_name: &str) -> Result<(), HomeErrors> {
        let pos = self.get_device_pos(device_name)?;
        self.devices.remove(pos);
        Ok(())
    }

    pub fn show_device(&self, device_name: &str) -> Result<String, HomeErrors> {
        let pos = self.get_device_pos(device_name)?;
        Ok(format!("{}", self.devices[pos]))
    }
}

impl fmt::Display for Room {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let room_data = &format!("Room : {}\n", self.name);
        fmt.write_str(room_data)?;
        for i in self.devices.iter() {
            let device_data = &format!("\t\tdevice : {}\n", i.get_printable());
            fmt.write_str(device_data)?;
        }
        Ok(())
    }
}

impl Clone for Room {
    fn clone(&self) -> Self {
        let mut c = Self {
            name: self.name.clone(),
            devices: Vec::new(),
        };
        for i in self.devices.iter() {
            let tmp = i.as_ref();
            c.devices.push(tmp.get_box());
        }
        return c;
    }

    fn clone_from(&mut self, source: &Self) {
        self.name = source.name.clone();
        self.devices = Vec::new();
        for i in source.devices.iter() {
            let tmp = i.as_ref();
            self.devices.push(tmp.get_box());
        }
    }
}

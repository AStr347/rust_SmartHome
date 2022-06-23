use ia_devices::device::TDevice;
use std::fmt;

pub struct Room {
    pub name: String,
    devices: Vec<Box<dyn TDevice>>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: Vec::new(),
        }
    }
    pub fn add_device(&mut self, device: impl TDevice) -> bool {
        let device_name = device.get_name();
        let pos = self.devices.iter().position(|x| -> bool {
            return *x.get_name() == *device_name;
        });
        match pos {
            Some(_) => {
                return false;
            }
            None => {
                let tmp = device.get_box();
                self.devices.push(tmp);
                return true;
            }
        }
    }
    pub fn remove_device(&mut self, device_name: &String) -> bool {
        let pos = self.devices.iter().position(|x| -> bool {
            return *x.get_name() == *device_name;
        });
        match pos {
            Some(x) => {
                self.devices.remove(x);
                return true;
            }
            None => {
                return false;
            }
        }
    }

    pub fn show_device(&self, device_name: &String){
        let pos = self.devices.iter().position(|x| -> bool {
            return *x.get_name() == *device_name;
        });
        match pos {
            Some(x) => {
                println!("{}", self.devices[x]);
            }
            None => {
                println!("have not deivce named {} inside room {}", device_name, self.name);
            }
        }
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

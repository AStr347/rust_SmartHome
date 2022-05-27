use std::{collections::LinkedList, fmt};
use ia_devices::devices::device::TDevice;


pub struct Room {
    pub name: String,
    devices: LinkedList<Box<dyn TDevice>>,
}

impl Room {
    fn exist_device(&self, name: &String) -> bool {
        for i in self.devices.iter() {
            if (i.get_name() == name) {
                return true;
            }
        }
        return false;
    }
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: LinkedList::new(),
        }
    }
    pub fn add_device(&mut self, device: impl TDevice) -> bool{
        let exist = self.exist_device(device.get_name());
        if(true == exist){
            return false;
        }
        let tmp = device.get_box();
        self.devices.push_back(tmp);
        return true;
    }
}


impl fmt::Display for  Room {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let room_data = &format!("Room : {}\n", self.name);
        fmt.write_str(room_data)?;
        for i in self.devices.iter(){
            let device_data = &format!("\t\tdevice : {}\n", i.get_status());
            fmt.write_str(device_data)?;
        }
        Ok(())
    }
}


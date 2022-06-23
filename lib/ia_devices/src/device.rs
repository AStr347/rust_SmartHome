use std::{collections::HashMap, fmt};

pub enum DeviceType {
    DtNone,
    DtSoket,
    DtWindow,
    DtWeather,
}

pub trait TDevice {
    fn get_name(&self) -> &String;
    fn get_printable(&self) -> String {
        return "".to_string();
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::DtNone;
    }
    fn get_box(&self) -> Box<dyn TDevice>;
    fn set_status(&mut self, args: &HashMap<&str, u64>);
    fn get_statuses(&self) -> HashMap<&str, u64>;
}

impl fmt::Display for dyn TDevice {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let device_data = &format!("device : {}", self.get_printable());
        fmt.write_str(device_data)?;
        Ok(())
    }
}

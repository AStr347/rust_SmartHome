use std::{collections::HashMap, fmt};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum DeviceType {
    None,
    Soket,
    Window,
    Weather,
}

pub trait TDevice {
    fn get_name(&self) -> &String;
    fn get_printable(&self) -> String {
        return "".to_string();
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::None;
    }
    fn get_box(&self) -> Box<dyn TDevice + Send>;
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

impl fmt::Display for dyn TDevice + Send {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let device_data = &format!("device : {}", self.get_printable());
        fmt.write_str(device_data)?;
        Ok(())
    }
}

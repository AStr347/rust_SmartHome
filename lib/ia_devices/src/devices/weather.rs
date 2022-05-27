use crate::devices::device::*;

#[derive(Copy, Clone)]
pub struct Weather {
    //pub name:String,
    pub temp:f32,
    pub hum:u8,
}

impl TDevice for Weather {
    fn get_name(&self) -> String {
        return "Weather".to_string();
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::DtWindow;
    }

    fn get_box(&self) -> Box<dyn TDevice> {
        return Box::new(self.clone());
    }
}
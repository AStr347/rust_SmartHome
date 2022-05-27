use crate::devices::device::*;

#[derive(Clone)]
pub struct Weather {
    //pub name:String,
    pub temp:f32,
    pub hum:u8,
}

impl TDevice for Weather {
    fn get_status(&self) -> String {
        return format!("it's {:20}\ttemp: {:5}\thum: {:5}%", "Smart Weather", self.temp, self.hum);
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::DtWindow;
    }

    fn get_box(&self) -> Box<dyn TDevice> {
        return Box::new(self.clone());
    }
    fn set_status(&mut self) {
        todo!()
    }
}

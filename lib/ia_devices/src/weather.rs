use crate::device::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Weather {
    pub name: String,
    pub temp: f32,
    pub hum: u8,
}

impl TDevice for Weather {
    fn get_name(&self) -> &String {
        return &self.name;
    }
    fn get_printable(&self) -> String {
        return format!(
            "it's {:50}\ttemp: {:5}\thum: {:5}%",
            self.name, self.temp, self.hum
        );
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::DtWindow;
    }
    fn get_box(&self) -> Box<dyn TDevice> {
        return Box::new(self.clone());
    }
    fn set_status(&mut self, args: &HashMap<&str, u64>) {
        let new_temp = args.get("temp");
        match new_temp {
            Some(x) => {
                self.temp = (*x as _);
            }
            None => {}
        }
        let new_hum = args.get("hum");
        match new_hum {
            Some(x) => {
                self.hum = (*x as _);
            }
            None => {}
        }
    }

    fn get_statuses(&self) -> HashMap<&str, u64> {
        return HashMap::from([("temp", self.temp as u64), ("hum", self.hum as u64)]);
    }
}

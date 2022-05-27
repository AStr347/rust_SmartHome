use crate::devices::device::*;

#[derive(Clone)]
pub struct Window {
    //pub name:String,
    pub pos:u8,
    pub ch:u8,
}

impl TDevice for Window {
    fn get_status(&self) -> String {
        return format!("it's {:20}\tposition: {:5}\tcharge: {:5}%", "Smart Window", self.pos, self.ch);
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
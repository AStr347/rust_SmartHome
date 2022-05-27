use crate::devices::device::*;

#[derive(Copy, Clone)]
pub struct Window {
    //pub name:String,
    pub pos:u8,
    pub ch:u8,
}

impl TDevice for Window {
    fn get_name(&self) -> String {
        return "Window".to_string();
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::DtWindow;
    }
    fn get_box(&self) -> Box<dyn TDevice> {
        return Box::new(self.clone());
    }
}
use crate::device::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Window {
    pub name: String,
    pub pos: u8,
    pub ch: u8,
}

impl TDevice for Window {
    fn get_name(&self) -> &String {
        return &self.name;
    }
    fn get_printable(&self) -> String {
        return format!(
            "it's {:50}\tposition: {:5}\tcharge: {:5}%",
            self.name, self.pos, self.ch
        );
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::Window;
    }
    fn get_box(&self) -> Box<dyn TDevice + Send> {
        Box::new(self.clone())
    }
    fn set_status(&mut self, args: &HashMap<&str, u64>) {
        let new_pos = args.get("pos");
        match new_pos {
            Some(x) => {
                self.pos = (*x as _);
            }
            None => {}
        }
        let new_ch = args.get("ch");
        match new_ch {
            Some(x) => {
                self.ch = (*x as _);
            }
            None => {}
        }
    }

    fn get_statuses(&self) -> HashMap<&str, u64> {
        return HashMap::from([("pos", self.pos as u64), ("ch", self.ch as u64)]);
    }
}

impl Window {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            pos: 0,
            ch: 100,
        }
    }
}

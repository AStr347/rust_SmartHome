use crate::device::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Soket {
    pub name: String,
    pub power: u32,
    pub state: bool,
}

impl TDevice for Soket {
    fn get_name(&self) -> &String {
        return &self.name;
    }
    fn get_printable(&self) -> String {
        let sstate = if self.state { "on" } else { "off" };
        return format!(
            "it's {:50}\tstate: {:5}\tpower: {:5}W",
            self.name, sstate, self.power
        );
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::Soket;
    }
    fn get_box(&self) -> Box<dyn TDevice + Send> {
        Box::new(self.clone())
    }
    fn set_status(&mut self, args: &HashMap<&str, u64>) {
        let new_state = args.get("state");
        match new_state {
            Some(1) => self.state = true,
            Some(0) => {
                self.state = false;
                self.power = 0;
            }
            _ => {}
        }
        if (true == self.state) {
            let new_power = args.get("power");
            match new_power {
                Some(x) => self.power = (*x as u32),
                None => self.power = 3000,
            }
        }
    }

    fn get_statuses(&self) -> HashMap<&str, u64> {
        return HashMap::from([("power", self.power as u64), ("state", self.state as u64)]);
    }
}

impl Soket {
    pub fn change_state(&mut self) {
        let state = self.state;
        if (true == state) {
            self.state = false;
            self.power = 0;
        } else {
            self.state = true;
            self.power = 3000;
        }
    }
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            power: 0,
            state: false,
        }
    }
}

#[cfg(test)]
mod soket_test {
    use super::*;
    #[test]
    fn test_TDevice() {
        let soket0 = Soket {
            name: "soket0".to_string(),
            power: 3000,
            state: true,
        };
        let device: &dyn device::TDevice = &soket0;
        println!("{}", device.get_name());
    }
}

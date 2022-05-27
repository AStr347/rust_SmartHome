use crate::devices::device::*;

#[derive(Clone)]
pub struct Soket {
    pub name:String,
    pub power:u32,
    pub state:bool,
}

impl TDevice for Soket {
    fn get_status(&self) -> String {
        let sstate = if self.state { "on" } else { "off" };
        return format!("it's {:50}\tstate: {:5}\tpower: {:5}W", self.name, sstate, self.power);
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::DtSoket;
    }
    fn get_box(&self) -> Box<dyn TDevice> {
        return Box::new(self.clone());
    }
    fn set_status(&mut self) {
        self.change_state();
    }
    fn get_name(&self) -> &String {
        return &self.name;
    }
}

impl Soket {
    pub fn change_state(&mut self){
        let state = self.state;
        if(true == state){
            self.state = false;
            self.power = 0;
        } else {
            self.state = true;
            self.power = 3000;
        }
    }
}

#[cfg(test)]
mod soket_test {
    use crate::soket::*;
    #[test]
    fn test_TDevice() {
        let soket0 = Soket{power:3000, state:true};
        let device : &dyn device::TDevice = &soket0;
        println!("{}", device.get_name());
    }
}
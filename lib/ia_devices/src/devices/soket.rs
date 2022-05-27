use crate::devices::device::*;

#[derive(Copy, Clone)]
pub struct Soket {
    //pub name:String,
    pub power:u32,
    pub state:bool,
}

impl TDevice for Soket {
    fn get_name(&self) -> String {
        return "Soket".to_string();
    }
    fn get_type(&self) -> DeviceType {
        return DeviceType::DtSoket;
    }
    fn get_box(&self) -> Box<dyn TDevice> {
        return Box::new(*self);
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
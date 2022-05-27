
pub enum DeviceType{
    DtNone,
    DtSoket,
    DtWindow,
    DtWeather,
}

pub trait TDevice{
    fn get_name(&self) -> &String;
    fn get_status(&self) -> String{
        return "".to_string();
    }
    fn get_type(&self) -> DeviceType{
        return DeviceType::DtNone;
    }
    fn get_box(&self) -> Box<dyn TDevice>;
    fn set_status(&mut self);
}

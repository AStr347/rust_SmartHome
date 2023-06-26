use soket::Soket;
use weather::Weather;
use window::Window;

pub mod device;
pub mod soket;
pub mod weather;
pub mod window;

use device::DeviceType;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeviceBuilder {
    name: String,
    dtype: DeviceType,
}

pub enum BuildedDevice {
    None,
    BuildedSoket(Soket),
    BuildedWindow(Window),
    BuildedWeather(Weather),
}

impl BuildedDevice {
    pub fn new(dtype: DeviceType, dname: String) -> Self {
        match dtype {
            DeviceType::None => Self::None,
            DeviceType::Soket => {
                let soket = Soket::new(dname);
                Self::BuildedSoket(soket)
            }
            DeviceType::Window => {
                let window = Window::new(dname);
                Self::BuildedWindow(window)
            }
            DeviceType::Weather => {
                let weather = Weather::new(dname);
                Self::BuildedWeather(weather)
            }
        }
    }
}

impl DeviceBuilder {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            dtype: DeviceType::None,
        }
    }
    
    pub fn name(mut self, name : String) -> Self {
        self.name = name;
        self
    }
    pub fn device_type(mut self, dtype: DeviceType) -> Self {
        self.dtype = dtype;
        self
    }

    pub fn build(self) -> BuildedDevice {
        BuildedDevice::new(self.dtype, self.name)
    }
}

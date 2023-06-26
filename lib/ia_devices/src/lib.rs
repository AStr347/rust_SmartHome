use soket::Soket;
use weather::Weather;
use window::Window;

pub mod device;
pub mod soket;
pub mod weather;
pub mod window;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeviceBuilder {
    name: String,
    dtype: device::DeviceType,
}

pub enum BuildedDevice {
    None,
    BuildedSoket(Soket),
    BuildedWindow(Window),
    BuildedWeather(Weather),
}

impl BuildedDevice {
    pub fn new(dtype: device::DeviceType, dname: String) -> Self {
        match dtype {
            device::DeviceType::None => Self::None,
            device::DeviceType::Soket => {
                let soket = Soket::new(dname);
                Self::BuildedSoket(soket)
            }
            device::DeviceType::Window => {
                let window = Window::new(dname);
                Self::BuildedWindow(window)
            }
            device::DeviceType::Weather => {
                let weather = Weather::new(dname);
                Self::BuildedWeather(weather)
            }
        }
    }
}

impl DeviceBuilder {
    pub fn build(self) -> BuildedDevice {
        BuildedDevice::new(self.dtype, self.name)
    }
}

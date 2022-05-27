use std::collections::LinkedList;
extern crate ia_devices;
use ia_devices::devices::device::*;
use ia_devices::devices::soket::*;

struct Room{
    name : String,
    devices : LinkedList<Box<dyn TDevice>>,
}

impl Room {
    fn _new(name: String) -> Self { Self { name, devices: LinkedList::new() } }
    fn _add_device(&mut self, device: &impl TDevice){
        let tmp = device.get_box();
        self.devices.push_back(tmp);
    }
}

struct Home{
    name : String,
    rooms : LinkedList<Room>,
}

impl Home {
    fn _new(name: String) -> Self { Self { name, rooms: LinkedList::new() } }
    fn _add_room(&mut self, room : Room) {
        self.rooms.push_back(room);
    }
}

fn main() {
    let soket0 = Soket{power:3000, state:true};
    println!("from soket {}", soket0.get_name());
    let device : &dyn TDevice = &soket0;
    println!("from device {}", device.get_name());
    // let soket1 = Soket{ power:0, state:false };

    // let mut room = Room::new("room0".to_string());
    // let device : &dyn TDevice = &soket0;

    // room.add_device(device);
}

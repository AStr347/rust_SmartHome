use std::collections::LinkedList;
extern crate ia_devices;
use ia_devices::devices::device::*;
use ia_devices::devices::soket::*;
use ia_devices::devices::weather::*;
use ia_devices::devices::window::*;

struct Room {
    devices: LinkedList<Box<dyn TDevice>>,
}

impl Room {
    fn new() -> Self {
        Self {
            devices: LinkedList::new(),
        }
    }
    fn add_device(&mut self, device: &impl TDevice) {
        let tmp = device.get_box();
        self.devices.push_back(tmp);
    }
}

struct Home {
    name: String,
    rooms: LinkedList<Room>,
}

impl Home {
    fn _new(name: String) -> Self {
        Self {
            name,
            rooms: LinkedList::new(),
        }
    }
    fn _add_room(&mut self, room: Room) {
        self.rooms.push_back(room);
    }
}

fn main() {
    let soket0 = Soket {
        power: 3000,
        state: true,
    };
    let soket1 = Soket {
        power: 0,
        state: false,
    };
    let weather = Weather {
        temp: 20.4,
        hum: 62,
    };
    let window = Window { pos: 75, ch: 98 };

    let mut room = Room::new();

    room.add_device(&soket0);
    room.add_device(&window);
    room.add_device(&weather);
    room.add_device(&soket1);

    for i in room.devices.iter() {
        println!("at room {}", i.get_status());
    }
}

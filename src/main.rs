extern crate ia_devices;
use ia_devices::devices::{soket::*, weather::*, window::*};

extern crate ia_home;
use ia_home::home::{home::*, room::*};

fn main() {
    let soket0 = Soket {
        name: "Soket0 inside 1st room".to_string(),
        power: 3000,
        state: true,
    };
    let soket1 = Soket {
        name: "Soket1 inside 1st room".to_string(),
        power: 0,
        state: false,
    };
    let weather = Weather {
        name: "Weather inside 1st room".to_string(),
        temp: 20.4,
        hum: 62,
    };
    let window = Window {
        name: "Window inside 1st room".to_string(),
        pos: 75,
        ch: 98,
    };

    let soket_d0 = Soket {
        name: "Soket0 inside 2nd room".to_string(),
        ..soket0
    };
    let soket_d1 = Soket {
        name: "Soket1 inside 2nd room".to_string(),
        ..soket0
    };
    let weather_d = Weather {
        name: "Weather inside 2nd room".to_string(),
        ..weather
    };
    let window_d = Window {
        name: "Window inside 2nd room".to_string(),
        ..window
    };

    let mut room = Room::new("1st room".to_string());
    room.add_device(soket0);
    room.add_device(window);
    room.add_device(weather);
    room.add_device(soket1);

    let mut home = Home::new("1st home".to_string());
    home.add_room(room);

    let mut room2 = Room::new("2nd room".to_string());
    room2.add_device(soket_d0);
    room2.add_device(soket_d1);
    room2.add_device(weather_d);
    room2.add_device(window_d);

    home.add_room(room2);

    println!("{}", home);
}

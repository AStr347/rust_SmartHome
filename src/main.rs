use std::{
    collections::HashMap,
    io::{self},
};

use ia_devices::{soket::*, weather::*, window::*};
use ia_home::{home::*, room::*};

fn read() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("failed to read from stdin");
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s
}

fn read_u32() -> Option<u32> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let trim = buffer.trim();
    match trim.parse::<u32>() {
        Ok(i) => Some(i),
        Err(..) => None,
    }
}

enum Commands<'a> {
    Exit,                                                //0
    AddRoom(String),                                     //1
    AddDevice(String),                                   //2
    RemoveRoom(String),                                  //3
    RemoveDevice(String, String),                        //4
    UpdateDevice(String, String, HashMap<&'a str, u64>), //5
    ShowHome,                                            //6
    ShowRoom(String),                                    //7
    ShowDevice(String, String),                          //8
    ShowInstruct,                                        //9
    Noop,
}

fn print_commands() {
    println!("\t\tcommands");
    println!("0: exit");
    println!("1: add room");
    println!("2: add device to room");
    println!("3: remove room");
    println!("4: remove device from room");
    println!("5: update device (todo)");
    println!("6: show home");
    println!("7: show room");
    println!("8: show device from room");
    println!("9: show that again");
    println!();
}

impl<'a> Commands<'a> {
    fn exec(&self, home: &mut Home) -> Self {
        match self {
            Commands::Noop => {
                let s = read_u32();
                match s {
                    Some(0) => {
                        return Commands::Exit;
                    }
                    Some(6) => return Commands::ShowHome,
                    Some(9) => {
                        return Commands::ShowInstruct;
                    }

                    Some(x) => {
                        println!("input room name:");
                        let roomname = read();
                        match x {
                            1 => {
                                return Commands::AddRoom(roomname);
                            }
                            2 => {
                                return Commands::AddDevice(roomname);
                            }
                            3 => {
                                return Commands::RemoveRoom(roomname);
                            }
                            7 => {
                                return Commands::ShowRoom(roomname);
                            }
                            y => {
                                println!("input device name:");
                                let devicename = read();
                                match y {
                                    4 => {
                                        return Commands::RemoveDevice(roomname, devicename);
                                    }
                                    8 => {
                                        return Commands::ShowDevice(roomname, devicename);
                                    }
                                    _ => {
                                        return Commands::Noop;
                                    } /* how it posible?? */
                                }
                            }
                        }
                    }
                    // Some(5) => {
                    //     let roomname = read();
                    //     let devicename = read();

                    //     return Commands::UpdateDevice(roomname, devicename, newstat);
                    // }
                    _ => {
                        return Commands::Noop;
                    }
                }
            }
            Commands::Exit => {
                return Commands::Exit;
            }
            Commands::AddRoom(roomname) => {
                let room = Room::new(roomname.clone());
                home.add_room(room);
            }
            Commands::AddDevice(roomname) => {
                println!("input device type (Soket, Window, Weather):");
                let devtype = read();
                let issoket = devtype.eq("Soket");
                let iswindow = devtype.eq("Window");
                let isweather = devtype.eq("Weather");

                if issoket || iswindow || isweather {
                    println!("input device name:");
                    let devname = read();
                    if issoket {
                        let soket = Soket {
                            name: devname,
                            power: 0,
                            state: false,
                        };
                        home.add_device(roomname, soket);
                    } else if iswindow {
                        let window = Window {
                            name: devname,
                            pos: 0,
                            ch: 100,
                        };
                        home.add_device(roomname, window);
                    } else if isweather {
                        let weather = Weather {
                            name: devname,
                            temp: 20.0,
                            hum: 60,
                        };
                        home.add_device(roomname, weather);
                    }
                }
            }
            Commands::RemoveRoom(roomname) => {
                home.remove_room(roomname);
            }
            Commands::RemoveDevice(roomname, devicename) => {
                home.remove_device(roomname, devicename);
            }
            Commands::UpdateDevice(_, _, _) => todo!(),
            Commands::ShowHome => {
                println!("{}\n", home);
            }
            Commands::ShowRoom(roomname) => {
                home.show_room(roomname);
            }
            Commands::ShowDevice(roomname, devicename) => {
                home.show_device(roomname, devicename);
            }
            Commands::ShowInstruct => {
                print_commands();
            }
        }
        
        Commands::Noop
    }
}

fn main() {
    let mut home = Home::new("1st home".to_string());
    let mut com = Commands::Noop;
    print_commands();
    loop {
        com = com.exec(&mut home);
        if let Commands::Exit = com {
            break;
        }
    }
}

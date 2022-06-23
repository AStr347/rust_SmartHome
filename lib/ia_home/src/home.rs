use std::fmt;

use ia_devices::device::TDevice;

use crate::room::Room;

pub struct Home {
    pub name: String,
    rooms: Vec<Box<Room>>,
}

impl Home {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: Vec::new(),
        }
    }
    pub fn add_room(&mut self, room: Room) -> bool {
        let room_name = &room.name;
        let pos = self
            .rooms
            .iter()
            .position(|x| -> bool { return (x.name == *room_name) });
        match pos {
            Some(_) => {
                return false;
            }
            None => {
                let tmp = Box::new(room.clone());
                self.rooms.push(tmp);
                return true;
            }
        }
    }
    pub fn add_device(&mut self, room_name: &String, device: impl TDevice) -> bool {
        let pos = self.rooms.iter().position(|x| -> bool {
            return x.name == *room_name;
        });
        match pos {
            Some(x) => {
                return self.rooms[x].add_device(device);
            }
            None => {
                return false;
            }
        }
    }

    pub fn remove_room(&mut self, room_name: &String) -> bool {
        let pos = self.rooms.iter().position(|x| -> bool {
            return x.name == *room_name;
        });
        match pos {
            Some(x) => {
                self.rooms.remove(x);
                return true;
            }
            None => {
                return false;
            }
        }
    }
    pub fn remove_device(&mut self, room_name: &String, device_name: &String) -> bool {
        let pos = self.rooms.iter().position(|x| -> bool {
            return x.name == *room_name;
        });
        match pos {
            Some(x) => {
                return self.rooms[x].remove_device(device_name);
            }
            None => {
                return false;
            }
        }
    }

    pub fn show_room(&self, room_name: &String){
        let pos = self.rooms.iter().position(|x| -> bool {
            return x.name == *room_name;
        });
        match pos {
            Some(x) => {
                println!("{}", self.rooms[x]);
            }
            _ => {
                println!("have not room named {}", room_name)
            }
        }
    }
    pub fn show_device(&self, room_name: &String, device_name: &String){
        let pos = self.rooms.iter().position(|x| -> bool {
            return x.name == *room_name;
        });
        match pos {
            Some(x) => {
                self.rooms[x].show_device(device_name);
            }
            _ => {
                println!("have not room named {} inside home {}", room_name, self.name);
            }
        }
    }
}

impl fmt::Display for Home {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let home_data = &format!("Home : {}\n", self.name);
        fmt.write_str(home_data)?;
        for i in self.rooms.iter() {
            let room_data = &format!("\t{}", i);
            fmt.write_str(room_data)?;
        }
        Ok(())
    }
}

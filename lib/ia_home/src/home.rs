use std::fmt;

use ia_devices::device::TDevice;

use crate::{homeerrors::HomeErrors, room::Room};

pub struct Home {
    pub name: String,
    rooms: Vec<Box<Room>>,
}

impl Home {
    fn get_room_pos(&self, room_name: &str) -> Result<usize, HomeErrors> {
        let pos = self
            .rooms
            .iter()
            .position(|x| -> bool { x.name == *room_name });
        if let Some(x) = pos {
            return Ok(x);
        }
        Err(HomeErrors::NotExist(room_name.to_string()))
    }

    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: Vec::new(),
        }
    }
    pub fn add_room(&mut self, room: Room) -> Result<(), HomeErrors> {
        let room_name = &room.name;
        let pos = self.get_room_pos(room_name);
        if let Ok(_) = pos {
            return Err(HomeErrors::AlreadyExist(room_name.clone()));
        }
        let tmp = Box::new(room.clone());
        self.rooms.push(tmp);
        Ok(())
    }
    pub fn add_device(&mut self, room_name: &str, device: impl TDevice) -> Result<(), HomeErrors> {
        let pos = self.get_room_pos(room_name)?;
        self.rooms[pos].add_device(device)
    }

    pub fn remove_room(&mut self, room_name: &str) -> Result<(), HomeErrors> {
        let pos = self.get_room_pos(room_name)?;
        self.rooms.remove(pos);
        Ok(())
    }
    pub fn remove_device(&mut self, room_name: &str, device_name: &str) -> Result<(), HomeErrors> {
        let pos = self.get_room_pos(room_name)?;
        self.rooms[pos].remove_device(device_name)
    }
    pub fn show_room(&self, room_name: &str) -> Result<String, HomeErrors> {
        let pos = self.get_room_pos(room_name)?;
        Ok(format!("{}", self.rooms[pos]))
    }
    pub fn show_device(&self, room_name: &str, device_name: &str) -> Result<String, HomeErrors> {
        let pos = self.get_room_pos(room_name)?;
        self.rooms[pos].show_device(device_name)
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

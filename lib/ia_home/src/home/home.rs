use std::{collections::LinkedList, fmt};

use crate::home::room::Room;

pub struct Home {
    pub name: String,
    rooms: LinkedList<Room>,
}

impl Home {
    fn exist_room(rooms: &LinkedList<Room>, name: &String) -> bool {
        for i in rooms.iter() {
            if (i.name == *name) {
                return true;
            }
        }
        return false;
    }

    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: LinkedList::new(),
        }
    }
    pub fn add_room(&mut self, room: Room) -> bool {
        let exist = Self::exist_room(&self.rooms, &room.name);
        if (true == exist) {
            return false;
        }
        self.rooms.push_back(room);
        return true;
    }
}

impl fmt::Display for  Home {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let home_data = &format!("Home : {}\n", self.name);
        fmt.write_str(home_data)?;
        for i in self.rooms.iter(){
            let room_data = &format!("\t{}", i);
            fmt.write_str(room_data)?;
        }
        Ok(())
    }
}

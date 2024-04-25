use std::time::{Duration, SystemTime};

pub struct GameDataHandler {
    data : GameData,
}

impl GameDataHandler {
    pub fn new(player_name: String) -> GameDataHandler {
        let data = GameData::new(player_name);
        GameDataHandler{data}
    }

    pub fn make_guess(&mut self){
        self.data.tries += 1;
    }

    pub fn player_name(&self) -> &String {
        &self.data.player_name
    }

    pub fn tries(&self) -> &u32 {
        &self.data.tries
    }

    pub fn time(&self) -> Duration {
        SystemTime::now()
        .duration_since(self.data.init_time)
        .expect("Euh bizarre le temps")
    }
}

pub struct GameData {
    pub player_name: String,
    pub tries: u32,
    pub init_time: SystemTime,
}

impl GameData {
    pub fn new(player_name: String) -> GameData {
        let tries: u32 = 0;
        let init_time = SystemTime::now();
        GameData{player_name, tries, init_time}
    }
}
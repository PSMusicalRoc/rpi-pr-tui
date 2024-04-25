pub mod player;

use std::path::PathBuf;

use player::Player;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PRSeason {
    num_brackets: u32,
    players: Vec<Player>
}

impl PRSeason {

    pub fn new() -> PRSeason {
        PRSeason {
            num_brackets: 0,
            players: Vec::new()
        }
    }

    pub fn save_to_file(&mut self, filename: String) -> Result<(), String> {

        self.check_representation().unwrap();

        // @TODO CHANGE THIS BACK TO THE UNPRETTY VERSION
        let self_json = match serde_json::to_string_pretty(self) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        match std::fs::write(filename, self_json) {
            Ok(_) => {},
            Err(e) => {
                return Err(format!("{}", e));
            }
        }

        Ok(())
    }

    pub fn load_from_file(filename: PathBuf) -> Result<PRSeason, String> {
        let instr = match std::fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        let retval: PRSeason = match serde_json::from_str(&instr) {
            Ok(pr) => pr,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        Ok(retval)
    }

    pub fn add_bracket(&mut self, _name: String, _num_entrants: u32) {
        self.num_brackets += 1;
    }

    pub fn add_player(&mut self, id: i32, tag: String) {
        self.players.push(Player::new(id, tag, self.num_brackets));
        self.players.last_mut().unwrap().change_number_of_brackets(self.num_brackets);
    }


    /* GETTERS */

    pub fn get_num_players(&self) -> u32 {
        self.players.len() as u32
    }

    pub fn get_num_brackets(&self) -> u32 {
        self.num_brackets
    }

    fn check_representation(&mut self) -> Result<(), String> {
        // Ensure that all players have the same number of placements as
        // there are brackets in the season.
        for player in self.players.iter_mut() {
            if player.get_number_brackets() != self.num_brackets {
                return Err(format!("Player {} should have {} brackets, actually has {}!",
                    player.get_tag(),
                    self.num_brackets,
                    player.get_number_brackets()
                ));
            }
        }
        
        Ok(())
    }
}
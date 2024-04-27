pub mod player;

use std::path::PathBuf;

use player::Player;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PRSeason {
    num_brackets: u32,
    players: Vec<Player>
}

fn is_placement_possible(placement: u32) -> bool {
    if placement == 1 || placement == 2 {
        return true;
    }

    let mut power: u32 = 4;
    let mut power_minus_one: u32 = 2;
    let mut power_minus_two: u32 = 1;

    while power_minus_two < 10_000 {
        let placement_one: u32 = power - power_minus_one + 1;
        let placement_two: u32 = power - power_minus_two + 1;

        if placement == placement_one || placement == placement_two {
            return true;
        }
        if placement < placement_two {
            return false;
        }

        power *= 2;
        power_minus_one *= 2;
        power_minus_two *= 2;
    }
    return false;
}

impl PRSeason {

    pub fn new() -> PRSeason {
        PRSeason {
            num_brackets: 0,
            players: Vec::new()
        }
    }

    pub fn save_to_file(&mut self, filename: PathBuf) -> Result<(), String> {

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

    pub fn get_player_vector(&self) -> &Vec<Player> {
        &self.players
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


#[test]
fn test_pr_placement_validity() {
    assert!(is_placement_possible(1));
    assert!(is_placement_possible(2));
    assert!(is_placement_possible(3));
    assert!(is_placement_possible(4));
    assert!(is_placement_possible(5));
    assert!(!is_placement_possible(6));
    assert!(is_placement_possible(7));
    assert!(!is_placement_possible(8));


    // larger values
    assert!(is_placement_possible(33));
    assert!(is_placement_possible(65));
    assert!(!is_placement_possible(4095));
    assert!(is_placement_possible(4097));
    assert!(!is_placement_possible(4098));
}
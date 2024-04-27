use serde::{Serialize, Deserialize};

/// The structure representing every player in
/// the PR software. It contains basic identification
/// information, as well as all placements
/// made by this player in brackets.
#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    /// The unique ID reserved by Start.GG for
    /// this player.
    id: i32,
    /// This player's tag, or online name.
    tag: String,
    /// A full list of all this player's placements
    /// in order, where a 0 means that they did
    /// not attend.
    placements: Vec<u32>
}


impl Player {

    pub fn new(id: i32, tag: String, num_brackets: u32) -> Player {
        Player {
            id,
            tag,
            placements: Vec::with_capacity(num_brackets as usize)
        }
    }

    pub fn change_number_of_brackets(&mut self, num: u32) {
        self.placements.resize(num as usize, 0);
    }


    /* GETTERS */

    pub fn get_number_brackets(&self) -> u32 {
        self.placements.len() as u32
    }

    pub fn get_tag(&self) -> String {
        self.tag.clone()
    }

    #[allow(dead_code)]
    pub fn get_start_gg_id(&self) -> i32 {
        self.id
    }

    #[allow(dead_code)]
    pub fn get_placements(&self) -> &Vec<u32> {
        &self.placements
    }

    /* SETTERS */

    #[allow(dead_code)]
    pub fn set_start_gg_id(&mut self, id: i32) {
        self.id = id;
    }

    #[allow(dead_code)]
    pub fn set_tag(&mut self, tag: String) {
        self.tag = tag;
    }

    #[allow(dead_code)]
    pub fn set_placement(&mut self, bracket_number: usize, placement: u32) -> Result<(), String> {
        if bracket_number >= self.placements.len() {
            return Err(format!("Bracket number {} is out of range!", bracket_number));
        }
        self.placements[bracket_number] = placement;

        Ok(())
    }
}

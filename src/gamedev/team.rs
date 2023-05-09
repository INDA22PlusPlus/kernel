use crate::gamedev::fighter::Fighter;

pub struct Team {
    pub fighting_team: [Option<Fighter>;6],                 // Struct to force first player? Lower to 5?
    pub collection: [Option<Fighter>;100]
}
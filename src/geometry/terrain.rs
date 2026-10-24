/// Terrain
pub enum Terrain {}

// --- Small types ---
pub struct TurnToken { 
    round: Round, 
    ship: ShipId, 
    nonce: u64 
}
impl TurnToken {
    //'nonce' is a unique number that a Ship mustn't be able to replicate
    pub fn issue(round: Round, ship: ShipId, nonce: u64) -> TurnToken{
        TurnToken{
            round,
            ship,
            nonce,
        }
    }

    pub fn round(self) -> Round{
        self.round
    }
}
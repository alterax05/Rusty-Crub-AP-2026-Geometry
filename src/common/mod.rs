//! Common types used across the code

/// `ShipID`
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct ShipId(pub u8);
/// `IslandID`
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct IslandId(pub u8);
/// Rounds are numbered from 1.
pub type Round = u32;
/// Reproducible randomness. Never the clock, never thread-local.
pub type Seed = u64;
/// `TurnToken`
pub struct TurnToken;
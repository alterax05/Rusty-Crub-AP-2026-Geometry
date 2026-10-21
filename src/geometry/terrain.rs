use crate::common::IslandId;

/// Describes the terrain occupying a map cell.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Terrain {
    /// Open water.
    Clear,
    /// Water, but foul. Entering costs half a full hull.
    Rock,
    /// Land, and part of an Island’s footprint.
    Island(IslandId),
    /// Land not owned by an island.
    Cliff,
}

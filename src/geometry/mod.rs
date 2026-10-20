//! Implementation of the hex geometry and algorithms (like the line of site) described in section 2 of the protocol document.
mod map;
pub use map::Map;
mod cell;
pub use cell::Cell;
mod direction;
pub use direction::Direction;
mod terrain;
pub use terrain::Terrain;
mod line_of_sight;
pub use line_of_sight::visible;


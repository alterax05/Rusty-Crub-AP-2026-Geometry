/// Represents one of the six directions on a hexagonal grid.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Direction {
    /// East direction.
    East,

    /// North-east direction.
    NorthEast,

    /// North-west direction.
    NorthWest,

    /// West direction.
    West,

    /// South-west direction.
    SouthWest,

    /// South-east direction.
    SouthEast,
}

impl Direction {
    /// Returns the axial coordinate offset for this direction.
    pub fn offset(self) -> (i32, i32) {
        match self {
            Direction::East => (1, 0),
            Direction::NorthEast => (1, -1),
            Direction::NorthWest => (0, -1),
            Direction::West => (-1, 0),
            Direction::SouthWest => (-1, 1),
            Direction::SouthEast => (0, 1),
        }
    }

    /// Returns the minimum angular distance in 60-degree steps.
    pub fn relative_angle(self, other: Direction) -> u8 {
        let a = self as i32;
        let b = other as i32;

        let diff = (a - b).abs();

        diff.min(6 - diff) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Checks that each of the six directions returns the correct axial coordinate offset.
    #[test]
    fn test_direction_offset() {
        assert_eq!(Direction::East.offset(), (1, 0));
        assert_eq!(Direction::NorthEast.offset(), (1, -1));
        assert_eq!(Direction::NorthWest.offset(), (0, -1));
        assert_eq!(Direction::West.offset(), (-1, 0));
        assert_eq!(Direction::SouthWest.offset(), (-1, 1));
        assert_eq!(Direction::SouthEast.offset(), (0, 1));
    }

    // Checks the relative angle for all 36 possible pairs of directions.
    #[test]
    fn test_all_relative_angles() {
        let directions = [
            Direction::East,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::West,
            Direction::SouthWest,
            Direction::SouthEast,
        ];

        let expected = [
            [0, 1, 2, 3, 2, 1],
            [1, 0, 1, 2, 3, 2],
            [2, 1, 0, 1, 2, 3],
            [3, 2, 1, 0, 1, 2],
            [2, 3, 2, 1, 0, 1],
            [1, 2, 3, 2, 1, 0],
        ];

        for i in 0..6 {
            for j in 0..6 {
                assert_eq!(
                    directions[i].relative_angle(directions[j]),
                    expected[i][j],
                    "Incorrect angle between {:?} and {:?}",
                    directions[i],
                    directions[j]
                );
            }
        }
    }
}

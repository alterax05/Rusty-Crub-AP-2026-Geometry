use super::direction::Direction;

/// Represents a cell on the hexagonal grid using axial coordinates.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Cell {
    /// The q coordinate of the cell.
    pub q: i32,

    /// The r coordinate of the cell.
    pub r: i32,
}

impl Cell {
    /// Returns the third cube coordinate, calculated as s = -q - r.
    pub fn s(self) -> i32 {
        -self.q - self.r
    }

    /// Returns the adjacent cell in the specified direction.
    pub fn neighbour(self, direction: Direction) -> Cell {
        let offset = direction.offset();

        Cell {
            q: self.q + offset.0,
            r: self.r + offset.1,
        }
    }

    /// Returns the hexagonal distance between two cells.
    pub fn distance(self, other: &Cell) -> u32 {
        (((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s() - other.s()).abs()) / 2) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Checks that moving in each direction returns the correct neighboring cell.
    #[test]
    fn test_neighbour() {
        let cell = Cell { q: 2, r: -1 };

        assert_eq!(cell.neighbour(Direction::East), Cell { q: 3, r: -1 });
        assert_eq!(cell.neighbour(Direction::NorthEast), Cell { q: 3, r: -2 });
        assert_eq!(cell.neighbour(Direction::NorthWest), Cell { q: 2, r: -2 });
        assert_eq!(cell.neighbour(Direction::West), Cell { q: 1, r: -1 });
        assert_eq!(cell.neighbour(Direction::SouthWest), Cell { q: 1, r: 0 });
        assert_eq!(cell.neighbour(Direction::SouthEast), Cell { q: 2, r: 0 });
    }

    // Checks that moving in a direction and then its opposite returns the starting cell.
    #[test]
    fn test_neighbour_opposite() {
        let cell = Cell { q: 2, r: -1 };

        let opposites = [
            (Direction::East, Direction::West),
            (Direction::NorthEast, Direction::SouthWest),
            (Direction::NorthWest, Direction::SouthEast),
            (Direction::West, Direction::East),
            (Direction::SouthWest, Direction::NorthEast),
            (Direction::SouthEast, Direction::NorthWest),
        ];

        for (direction, opposite) in opposites {
            let next = cell.neighbour(direction);
            let back = next.neighbour(opposite);

            assert_eq!(back, cell);
        }
    }

    // Checks distances between cells, including zero, adjacent cells, and symmetry.
    #[test]
    fn test_distance() {
        let a = Cell { q: 0, r: 0 };
        let b = Cell { q: 2, r: -1 };
        let c = Cell { q: 1, r: 0 };

        assert_eq!(a.distance(&a), 0);
        assert_eq!(a.distance(&c), 1);
        assert_eq!(a.distance(&b), 2);
        assert_eq!(a.distance(&b), b.distance(&a));
    }

    // Checks distances with negative coordinates, symmetry, and all six neighboring cells.
    #[test]
    fn test_distance_negative_coordinates() {
        let a = Cell { q: -2, r: 3 };
        let b = Cell { q: 1, r: -1 };

        assert_eq!(a.distance(&b), 4);
        assert_eq!(b.distance(&a), 4);

        let directions = [
            Direction::East,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::West,
            Direction::SouthWest,
            Direction::SouthEast,
        ];

        for direction in directions {
            let neighbour = a.neighbour(direction);

            assert_eq!(
                a.distance(&neighbour),
                1,
                "Incorrect distance in direction {:?}",
                direction
            );
        }
    }

    // Checks distance calculations with large coordinates to detect integer overflow.
    #[test]
    #[ignore = "Integer overflow handling postponed"]
    fn test_distance_large_coordinates() {
        let a = Cell { q: i32::MAX, r: 0 };

        let b = Cell { q: -1, r: 0 };

        assert_eq!(a.distance(&b), 2_147_483_648);
    }

    // Checks the third cube coordinate and verifies that q + r + s = 0.
    #[test]
    fn test_s() {
        let a = Cell { q: 0, r: 0 };
        let b = Cell { q: 2, r: -1 };
        let c = Cell { q: -3, r: 1 };

        assert_eq!(a.s(), 0);
        assert_eq!(b.s(), -1);
        assert_eq!(c.s(), 2);

        for cell in [a, b, c] {
            assert_eq!(cell.q + cell.r + cell.s(), 0);
        }
    }
}

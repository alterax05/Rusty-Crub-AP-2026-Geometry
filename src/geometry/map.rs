use std::collections::BTreeMap;

use super::cell::Cell;
use super::terrain::Terrain;

/// The terrain of every cell, and nothing else. Public and fixed for the game.
pub struct Map {
    radius: u32,
    terrain: BTreeMap<Cell, Terrain>,
}

impl Map {
    /// Returns the hexagonal map radius.
    pub fn radius(&self) -> u32 {
        self.radius
    }
    /// Returns whether a cell lies inside the hexagonal map boundaries.
    pub fn contains(&self, cell: Cell) -> bool {
        let q = cell.q;
        let r = cell.r;
        let s = -q - r;

        q.unsigned_abs() <= self.radius && r.unsigned_abs() <= self.radius && s.unsigned_abs() <= self.radius
    }
    /// Returns the terrain of a cell, or None if the cell is absent.
    pub fn terrain(&self, cell: Cell) -> Option<Terrain> {
        self.terrain.get(&cell).cloned()
    }
    /// Returns whether the cell contains an island or cliff.
    pub fn is_land(&self, cell: Cell) -> bool {
        match self.terrain.get(&cell) {
            Some(Terrain::Island(_)) | Some(Terrain::Cliff) => true,
            _ => false,
        }
    }
    /// Returns whether the cell contains clear water or rocks.
    pub fn is_water(&self, cell: Cell) -> bool {
        match self.terrain.get(&cell) {
            Some(Terrain::Rock) | Some(Terrain::Clear) => true,
            _ => false,
        }
    }
    /// The six corners, in direction order. Ship i starts on corner i.
    pub fn corners(&self) -> [Cell; 6] {
        let rad = self.radius as i32;
        [
            Cell { q: rad, r: 0 },
            Cell { q: rad, r: -rad },
            Cell { q: 0, r: -rad },
            Cell { q: -rad, r: 0 },
            Cell { q: -rad, r: rad },
            Cell { q: 0, r: rad },
        ]
    }

    /// Iterates over water cells (clear water and rocks) in sorted order.
    pub fn water_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        self.terrain.iter().filter_map(|(&cell, &terrain)| match terrain {
            Terrain::Clear | Terrain::Rock => Some(cell),
            _ => None,
        })
    }

    /// Iterates over clear-water cells in sorted order.
    pub fn clear_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        self.terrain.iter().filter_map(|(&cell, &terrain)| match terrain {
            Terrain::Clear => Some(cell),
            _ => None,
        })
    }
}

// Unit-test-only helpers: preserve the production Map API and field privacy.
#[cfg(test)]
impl Map {
    /// Builds a map with a chosen terrain layout for unit tests.
    pub(crate) fn for_tests(radius: u32, terrain: BTreeMap<Cell, Terrain>) -> Self {
        Self { radius, terrain }
    }

    /// Changes a terrain cell in a unit test.
    pub(crate) fn set_terrain_for_tests(&mut self, cell: Cell, terrain: Terrain) {
        self.terrain.insert(cell, terrain);
    }
}

#[cfg(test)]
mod map_tests {
    use super::super::direction::Direction;
    use super::*;
    use crate::common::IslandId;

    // Checks whether cells are correctly identified as inside or outside the map.
    #[test]
    fn test_contains() {
        let map = Map {
            radius: 3,
            terrain: BTreeMap::new(),
        };

        // Celle dentro la mappa
        assert!(map.contains(Cell { q: 0, r: 0 }));
        assert!(map.contains(Cell { q: 3, r: 0 }));
        assert!(map.contains(Cell { q: 3, r: -3 }));
        assert!(map.contains(Cell { q: -3, r: 0 }));
        assert!(map.contains(Cell { q: 0, r: 3 }));

        // Celle fuori dalla mappa
        assert!(!map.contains(Cell { q: 4, r: 0 }));
        assert!(!map.contains(Cell { q: 2, r: 2 }));
        assert!(!map.contains(Cell { q: -4, r: 1 }));
    }

    // Checks that each cell returns the correct terrain type or None if absent.
    #[test]
    fn test_terrain() {
        let mut terrain = BTreeMap::new();

        // Mappa completa di raggio 1
        terrain.insert(Cell { q: 0, r: 0 }, Terrain::Clear);
        terrain.insert(Cell { q: 1, r: 0 }, Terrain::Rock);
        terrain.insert(Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1)));
        terrain.insert(Cell { q: 0, r: -1 }, Terrain::Cliff);
        terrain.insert(Cell { q: -1, r: 0 }, Terrain::Clear);
        terrain.insert(Cell { q: -1, r: 1 }, Terrain::Clear);
        terrain.insert(Cell { q: 0, r: 1 }, Terrain::Clear);

        let map = Map { radius: 1, terrain };

        // Verifichiamo i quattro tipi di terreno
        assert_eq!(map.terrain(Cell { q: 0, r: 0 }), Some(Terrain::Clear));

        assert_eq!(map.terrain(Cell { q: 1, r: 0 }), Some(Terrain::Rock));

        assert_eq!(map.terrain(Cell { q: 1, r: -1 }), Some(Terrain::Island(IslandId(1))));

        assert_eq!(map.terrain(Cell { q: 0, r: -1 }), Some(Terrain::Cliff));

        // Cella fuori dalla mappa
        assert_eq!(map.terrain(Cell { q: 2, r: 0 }), None);
    }

    // Checks that Island and Cliff are classified as land, while other cells are not.
    #[test]
    fn test_is_land() {
        let terrain = BTreeMap::from([
            (Cell { q: 0, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: 0 }, Terrain::Rock),
            (Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: 0, r: -1 }, Terrain::Cliff),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ]);

        let map = Map { radius: 1, terrain };

        // Island e Cliff sono terra
        assert!(map.is_land(Cell { q: 1, r: -1 }));
        assert!(map.is_land(Cell { q: 0, r: -1 }));

        // Clear e Rock sono acqua
        assert!(!map.is_land(Cell { q: 0, r: 0 }));
        assert!(!map.is_land(Cell { q: 1, r: 0 }));

        // Cella fuori dalla mappa
        assert!(!map.is_land(Cell { q: 2, r: 0 }));
    }

    // Checks that Clear and Rock are classified as water, while other cells are not.
    #[test]
    fn test_is_water() {
        let terrain = BTreeMap::from([
            (Cell { q: 0, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: 0 }, Terrain::Rock),
            (Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: 0, r: -1 }, Terrain::Cliff),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ]);

        let map = Map { radius: 1, terrain };

        // Clear e Rock sono acqua
        assert!(map.is_water(Cell { q: 0, r: 0 }));
        assert!(map.is_water(Cell { q: 1, r: 0 }));

        // Island e Cliff non sono acqua
        assert!(!map.is_water(Cell { q: 1, r: -1 }));
        assert!(!map.is_water(Cell { q: 0, r: -1 }));

        // Cella fuori dalla mappa
        assert!(!map.is_water(Cell { q: 2, r: 0 }));
    }

    // Checks the coordinates and order of the six map corners for a radius of 3.
    #[test]
    fn test_corners() {
        let map = Map {
            radius: 3,
            terrain: BTreeMap::new(),
        };

        let expected = [
            Cell { q: 3, r: 0 },
            Cell { q: 3, r: -3 },
            Cell { q: 0, r: -3 },
            Cell { q: -3, r: 0 },
            Cell { q: -3, r: 3 },
            Cell { q: 0, r: 3 },
        ];

        // Verifica che i sei angoli siano corretti e ordinati
        assert_eq!(map.corners(), expected);

        // Verifica che tutti gli angoli appartengano alla mappa
        for corner in map.corners() {
            assert!(map.contains(corner));
        }
    }

    // Checks that the map returns the correct radius for different values.
    #[test]
    fn test_radius() {
        let radii = [0, 1, 3, 5, 10];

        for radius in radii {
            let map = Map {
                radius,
                terrain: BTreeMap::new(),
            };

            assert_eq!(map.radius(), radius);
        }
    }

    // Checks that each existing cell is either land or water, but never both.
    #[test]
    fn test_land_water_exclusive() {
        let terrain = BTreeMap::from([
            (Cell { q: 0, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: 0 }, Terrain::Rock),
            (Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: 0, r: -1 }, Terrain::Cliff),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ]);

        let map = Map { radius: 1, terrain };

        // Ogni cella deve essere terra oppure acqua
        for &cell in map.terrain.keys() {
            assert_ne!(
                map.is_land(cell),
                map.is_water(cell),
                "Classificazione errata per {:?}",
                cell
            );
        }

        // Una cella inesistente non è né terra né acqua
        let outside = Cell { q: 2, r: 0 };

        assert!(!map.is_land(outside));
        assert!(!map.is_water(outside));
    }

    // Checks map boundaries for different radii, including the third cube coordinate.
    #[test]
    fn test_contains_multiple_radii() {
        let radii = [0, 1, 2, 3, 5];

        for radius in radii {
            let map = Map {
                radius,
                terrain: BTreeMap::new(),
            };

            let r = radius as i32;

            // L'origine appartiene sempre alla mappa
            assert!(map.contains(Cell { q: 0, r: 0 }));

            // Tutti i sei angoli devono appartenere alla mappa
            for corner in map.corners() {
                assert!(
                    map.contains(corner),
                    "Angolo {:?} non valido per raggio {}",
                    corner,
                    radius
                );
            }

            // Celle oltre il raggio
            assert!(!map.contains(Cell { q: r + 1, r: 0 }));
            assert!(!map.contains(Cell { q: 0, r: -r - 1 }));

            // Entrambe le coordinate possono essere
            // entro il raggio, ma s può essere fuori
            if radius > 0 {
                assert!(!map.contains(Cell { q: r, r }));
            }
        }
    }

    // Checks corner positions, order, distance, and uniqueness for different radii.
    #[test]
    fn test_corners_multiple_radii() {
        let radii = [1, 2, 3, 5, 10];

        let directions = [
            Direction::East,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::West,
            Direction::SouthWest,
            Direction::SouthEast,
        ];

        let origin = Cell { q: 0, r: 0 };

        for radius in radii {
            let map = Map {
                radius,
                terrain: BTreeMap::new(),
            };

            let corners = map.corners();

            for (i, direction) in directions.iter().enumerate() {
                let (dq, dr) = direction.offset();

                let expected = Cell {
                    q: dq * radius as i32,
                    r: dr * radius as i32,
                };

                // Verifica la posizione e l'ordine
                assert_eq!(corners[i], expected);

                // Verifica la distanza dall'origine
                assert_eq!(origin.distance(&corners[i]), radius);
            }

            // I sei angoli devono essere distinti
            for i in 0..6 {
                for j in (i + 1)..6 {
                    assert_ne!(corners[i], corners[j]);
                }
            }
        }
    }

    // Checks that water_cells returns only Clear and Rock cells in a deterministic order.
    #[test]
    fn test_water_cells() {
        let terrain = BTreeMap::from([
            (Cell { q: 0, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: 0 }, Terrain::Rock),
            (Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: 0, r: -1 }, Terrain::Cliff),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ]);

        let map = Map { radius: 1, terrain };

        let water: Vec<Cell> = map.water_cells().collect();

        let expected = vec![
            Cell { q: -1, r: 0 },
            Cell { q: -1, r: 1 },
            Cell { q: 0, r: 0 },
            Cell { q: 0, r: 1 },
            Cell { q: 1, r: 0 },
        ];

        assert_eq!(water, expected);
    }

    // Checks that clear_cells returns only Clear cells in a deterministic order.
    #[test]
    fn test_clear_cells() {
        let terrain = BTreeMap::from([
            (Cell { q: 0, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: 0 }, Terrain::Rock),
            (Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: 0, r: -1 }, Terrain::Cliff),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ]);

        let map = Map { radius: 1, terrain };

        let clear: Vec<Cell> = map.clear_cells().collect();

        let expected = vec![
            Cell { q: -1, r: 0 },
            Cell { q: -1, r: 1 },
            Cell { q: 0, r: 0 },
            Cell { q: 0, r: 1 },
        ];

        assert_eq!(clear, expected);
    }
}

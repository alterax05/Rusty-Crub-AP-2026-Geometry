use super::cell::Cell;
use super::map::Map;

// Computes the squared geometric distance between a cell center and a fractional sample point using only integer arithmetic.
fn squared_distance_to_sample(cell: Cell, q_num: i64, r_num: i64, n: i64) -> i64 {
    // Convert the cell coordinates to cube coordinates.
    let q = i64::from(cell.q);
    let r = i64::from(cell.r);
    let s = -q - r;

    // Calculate the third numerator of the fractional sample.
    let s_num = -q_num - r_num;

    // Calculate coordinate differences without dividing by n.
    let dq = q * n - q_num;
    let dr = r * n - r_num;
    let ds = s * n - s_num;

    // Return the squared distance scaled by n^2.
    // This preserves exact comparisons without floating-point errors.
    dq * dq + dr * dr + ds * ds
}

// Finds all cells whose centers are closest to a fractional sample using exact integer comparisons without floating-point rounding.
fn nearest_candidates(q_num: i64, r_num: i64, n: i64) -> Vec<Cell> {
    assert!(n > 0);

    // Find the lower and upper integer values of q.
    let q0 = q_num.div_euclid(n);
    let q1 = q0 + if q_num.rem_euclid(n) == 0 { 0 } else { 1 };

    // Find the lower and upper integer values of r.
    let r0 = r_num.div_euclid(n);
    let r1 = r0 + if r_num.rem_euclid(n) == 0 { 0 } else { 1 };

    // Store the closest cells and the minimum distance found.
    let mut candidates = Vec::new();
    let mut min_distance = i64::MAX;

    // Check all combinations of nearby integer coordinates.
    for q in [q0, q1] {
        for r in [r0, r1] {
            // Convert coordinates back to the Cell type (i32).
            let (Ok(q), Ok(r)) = (i32::try_from(q), i32::try_from(r)) else {
                continue;
            };

            let cell = Cell { q, r };

            // Calculate the exact squared distance to the sample.
            let distance = squared_distance_to_sample(cell, q_num, r_num, n);

            if distance < min_distance {
                // A closer cell was found: replace previous candidates.
                min_distance = distance;
                candidates.clear();
                candidates.push(cell);
            } else if distance == min_distance && !candidates.contains(&cell) {
                // Keep equally close cells without adding duplicates.
                candidates.push(cell);
            }
        }
    }

    // Return all cells sharing the minimum distance.
    candidates
}

// Computes the exact numerators of a sample point along a line.
// The coordinates share denominator n, avoiding floating-point errors.
fn sample_numerators(from: Cell, to: Cell, k: i64, n: i64) -> (i64, i64) {
    assert!(n > 0);
    assert!(k >= 0 && k <= n);

    // Interpolate the q coordinate without dividing by n.
    let q_num = (n - k) * i64::from(from.q) + k * i64::from(to.q);

    // Interpolate the r coordinate without dividing by n.
    let r_num = (n - k) * i64::from(from.r) + k * i64::from(to.r);

    // Return the two numerators.
    (q_num, r_num)
}

/// Returns whether two cells have unobstructed line of sight within the given range.
/// Land between the endpoints blocks visibility only if every equally close cell at an interior sample is land. Endpoints do not block visibility.
#[must_use]
pub fn visible(map: &Map, from: Cell, to: Cell, vision_range: u32) -> bool {
    let distance = from.distance(&to);

    if distance > vision_range {
        return false;
    }

    let n = i64::from(distance);

    // Check only interior samples, excluding both endpoints.
    for k in 1..n {
        // Compute the exact fractional sample coordinates.
        let (q_num, r_num) = sample_numerators(from, to, k, n);

        // Find all cells equally closest to the sample.
        let candidates = nearest_candidates(q_num, r_num, n);

        // A sample blocks sight only when all candidates are land.
        if candidates.iter().all(|&cell| map.is_land(cell)) {
            return false;
        }
    }

    // No blocking samples were found.
    true
}

#[cfg(test)]
mod los_tests {
    use super::super::terrain::Terrain;
    use super::*;
    use crate::common::IslandId;
    use std::collections::BTreeMap;

    // Checks that two cells are exactly equidistant from a midpoint.
    #[test]
    fn test_squared_distance_midpoint() {
        let a = Cell { q: 1, r: -1 };
        let b = Cell { q: 1, r: 0 };

        // Midpoint (1, -0.5, -0.5)
        let q_num = 2;
        let r_num = -1;
        let n = 2;

        let distance_a = squared_distance_to_sample(a, q_num, r_num, n);

        let distance_b = squared_distance_to_sample(b, q_num, r_num, n);

        assert_eq!(distance_a, 2);
        assert_eq!(distance_b, 2);
        assert_eq!(distance_a, distance_b);
    }

    // Checks that an exact midpoint returns both nearest cells.
    #[test]
    fn test_nearest_candidates_midpoint() {
        // Point (1, -0.5, -0.5) represented exactly.
        let q_num = 2;
        let r_num = -1;
        let n = 2;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.len(), 2);

        assert!(candidates.contains(&Cell { q: 1, r: -1 }));
        assert!(candidates.contains(&Cell { q: 1, r: 0 }));
    }

    // Checks exact interpolation of sample points using integer numerators.
    #[test]
    fn test_sample_numerators() {
        let from = Cell { q: 0, r: 0 };
        let to = Cell { q: 2, r: 1 };

        let n = i64::from(from.distance(&to));

        assert_eq!(n, 3);

        // First intermediate sample: (2/3, 1/3)
        assert_eq!(sample_numerators(from, to, 1, n), (2, 1));

        // Second intermediate sample: (4/3, 2/3)
        assert_eq!(sample_numerators(from, to, 2, n), (4, 2));

        // The endpoints must match the original cells.
        assert_eq!(sample_numerators(from, to, 0, n), (0, 0));
        assert_eq!(sample_numerators(from, to, 3, n), (6, 3));
    }

    // Checks that a fractional sample has exactly one nearest cell.
    #[test]
    fn test_nearest_candidates_single() {
        let q_num = 2;
        let r_num = 1;
        let n = 3;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0], Cell { q: 1, r: 0 });
    }

    // Checks that negative fractional coordinates return the correct nearest cell.
    #[test]
    fn test_nearest_candidates_negative() {
        // Point (-4/3, 1/3, 1) represented exactly.
        let q_num = -4;
        let r_num = 1;
        let n = 3;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0], Cell { q: -1, r: 0 });
    }

    // Checks that a point on a shared vertex returns all three nearest cells.
    #[test]
    fn test_nearest_candidates_three_cells() {
        // Point (2/3, -1/3, -1/3) represented exactly.
        let q_num = 2;
        let r_num = -1;
        let n = 3;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.len(), 3);

        assert!(candidates.contains(&Cell { q: 0, r: 0 }));
        assert!(candidates.contains(&Cell { q: 1, r: -1 }));
        assert!(candidates.contains(&Cell { q: 1, r: 0 }));
    }

    // Checks that a cell can see itself even with zero vision range and land terrain.
    #[test]
    fn test_visible_same_cell() {
        let cell = Cell { q: 0, r: 0 };

        let terrain = BTreeMap::from([(cell, Terrain::Island(IslandId(1)))]);

        let map = Map::for_tests(0, terrain);

        assert!(visible(&map, cell, cell, 0));
    }

    // Checks that cells beyond the maximum vision range are not visible.
    #[test]
    fn test_visible_out_of_range() {
        let from = Cell { q: 0, r: 0 };
        let to = Cell { q: 2, r: 0 };

        let map = Map::for_tests(2, BTreeMap::new());

        // The distance is 2, but the vision range is only 1.
        assert_eq!(from.distance(&to), 2);
        assert!(!visible(&map, from, to, 1));
    }

    // Checks that two cells are visible when the line between them contains only clear water.
    #[test]
    fn test_visible_clear_path() {
        let from = Cell { q: -1, r: 0 };
        let to = Cell { q: 1, r: 0 };

        // Complete radius-1 map with only clear water.
        let terrain = BTreeMap::from([
            (Cell { q: 0, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: -1 }, Terrain::Clear),
            (Cell { q: 0, r: -1 }, Terrain::Clear),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ]);

        let map = Map::for_tests(1, terrain);

        assert_eq!(from.distance(&to), 2);

        // Both cells are exactly within the vision range.
        assert!(visible(&map, from, to, 2));
    }

    // Checks that an island directly between two cells blocks line of sight.
    #[test]
    fn test_visible_land_blocked() {
        let from = Cell { q: -1, r: 0 };
        let to = Cell { q: 1, r: 0 };

        let terrain = BTreeMap::from([
            (Cell { q: 0, r: 0 }, Terrain::Island(IslandId(1))),
            (Cell { q: 1, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: -1 }, Terrain::Clear),
            (Cell { q: 0, r: -1 }, Terrain::Clear),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ]);

        let map = Map::for_tests(1, terrain);

        assert_eq!(from.distance(&to), 2);

        // The island in the middle must block visibility.
        assert!(!visible(&map, from, to, 2));
    }

    // Checks that a cliff directly between two cells blocks line of sight.
    #[test]
    fn test_visible_cliff_blocked() {
        let from = Cell { q: -1, r: 0 };
        let to = Cell { q: 1, r: 0 };

        let terrain = BTreeMap::from([
            (Cell { q: 0, r: 0 }, Terrain::Cliff),
            (Cell { q: 1, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: -1 }, Terrain::Clear),
            (Cell { q: 0, r: -1 }, Terrain::Clear),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ]);

        let map = Map::for_tests(1, terrain);

        assert_eq!(from.distance(&to), 2);

        // The cliff in the middle must block visibility.
        assert!(!visible(&map, from, to, 2));
    }

    // Checks that a rock between two cells does not block line of sight.
    #[test]
    fn test_visible_rock() {
        let from = Cell { q: -1, r: 0 };
        let to = Cell { q: 1, r: 0 };

        let terrain = BTreeMap::from([
            (Cell { q: 0, r: 0 }, Terrain::Rock),
            (Cell { q: 1, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: -1 }, Terrain::Clear),
            (Cell { q: 0, r: -1 }, Terrain::Clear),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ]);

        let map = Map::for_tests(1, terrain);

        assert_eq!(from.distance(&to), 2);

        // Rock is water and must not block visibility.
        assert!(visible(&map, from, to, 2));
    }

    // Checks that a line passing between land and water remains visible.
    #[test]
    fn test_visible_boundary_land_water() {
        let from = Cell { q: 0, r: 0 };
        let to = Cell { q: 2, r: -1 };

        // Create a complete radius-2 map with clear water.
        let mut terrain = BTreeMap::new();

        for q in -2_i32..=2 {
            for r in -2_i32..=2 {
                if (-q - r).abs() <= 2 {
                    terrain.insert(Cell { q, r }, Terrain::Clear);
                }
            }
        }

        // The line passes exactly between these two cells.
        terrain.insert(Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1)));

        terrain.insert(Cell { q: 1, r: 0 }, Terrain::Clear);

        let map = Map::for_tests(2, terrain);

        assert_eq!(from.distance(&to), 2);

        // One candidate is water, so visibility must not be blocked.
        assert!(visible(&map, from, to, 2));
    }

    // Checks that a line passing between two land cells is blocked.
    #[test]
    fn test_visible_boundary_land_land() {
        let from = Cell { q: 0, r: 0 };
        let to = Cell { q: 2, r: -1 };

        // Create a complete radius-2 map with clear water.
        let mut terrain = BTreeMap::new();

        for q in -2_i32..=2 {
            for r in -2_i32..=2 {
                if (-q - r).abs() <= 2 {
                    terrain.insert(Cell { q, r }, Terrain::Clear);
                }
            }
        }

        // Both cells sharing the boundary are land.
        terrain.insert(Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1)));

        terrain.insert(Cell { q: 1, r: 0 }, Terrain::Cliff);

        let map = Map::for_tests(2, terrain);

        assert_eq!(from.distance(&to), 2);

        // Both nearest candidates are land, so visibility is blocked.
        assert!(!visible(&map, from, to, 2));
    }

    // Checks that reversing the line of sight always gives the same visibility result.
    #[test]
    fn test_visible_symmetry() {
        let a = Cell { q: 0, r: 0 };
        let b = Cell { q: 2, r: -1 };

        // Create a complete radius-2 map with clear water.
        let mut terrain = BTreeMap::new();

        for q in -2_i32..=2 {
            for r in -2_i32..=2 {
                if (-q - r).abs() <= 2 {
                    terrain.insert(Cell { q, r }, Terrain::Clear);
                }
            }
        }

        // First case: one land cell and one water cell.
        terrain.insert(Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1)));

        let mut map = Map::for_tests(2, terrain);

        // Visibility must work in both directions.
        assert!(visible(&map, a, b, 2));
        assert!(visible(&map, b, a, 2));

        // Second case: both intermediate candidates are land.
        map.set_terrain_for_tests(Cell { q: 1, r: 0 }, Terrain::Cliff);

        // Visibility must be blocked in both directions.
        assert!(!visible(&map, a, b, 2));
        assert!(!visible(&map, b, a, 2));
    }

    // Checks that land at the starting and ending cells does not block line of sight.
    #[test]
    fn test_visible_endpoints() {
        let from = Cell { q: 0, r: 0 };
        let to = Cell { q: 2, r: 0 };

        // Create a complete radius-2 map with clear water.
        let mut terrain = BTreeMap::new();

        for q in -2_i32..=2 {
            for r in -2_i32..=2 {
                if (-q - r).abs() <= 2 {
                    terrain.insert(Cell { q, r }, Terrain::Clear);
                }
            }
        }

        // Both endpoints are land.
        terrain.insert(from, Terrain::Cliff);
        terrain.insert(to, Terrain::Island(IslandId(1)));

        let map = Map::for_tests(2, terrain);

        assert_eq!(from.distance(&to), 2);

        // Land endpoints must not block visibility.
        assert!(visible(&map, from, to, 2));
        assert!(visible(&map, to, from, 2));
    }

    // Checks that visibility includes cells exactly at the maximum vision range.
    #[test]
    fn test_visible_range_boundary() {
        let from = Cell { q: 0, r: 0 };
        let to = Cell { q: 2, r: 0 };

        // Create a complete radius-2 map with clear water.
        let mut terrain = BTreeMap::new();

        for q in -2_i32..=2 {
            for r in -2_i32..=2 {
                if (-q - r).abs() <= 2 {
                    terrain.insert(Cell { q, r }, Terrain::Clear);
                }
            }
        }

        let map = Map::for_tests(2, terrain);

        assert_eq!(from.distance(&to), 2);

        // Distance exceeds the vision range.
        assert!(!visible(&map, from, to, 1));

        // Distance is exactly equal to the vision range.
        assert!(visible(&map, from, to, 2));

        // Distance is smaller than the vision range.
        assert!(visible(&map, from, to, 3));
    }

    // Checks that every interior sample is examined along a longer line of sight.
    #[test]
    fn test_visible_multiple_samples() {
        let from = Cell { q: -2, r: 0 };
        let to = Cell { q: 2, r: 0 };

        // Create a complete radius-2 map with clear water.
        let mut terrain = BTreeMap::new();

        for q in -2_i32..=2 {
            for r in -2_i32..=2 {
                if (-q - r).abs() <= 2 {
                    terrain.insert(Cell { q, r }, Terrain::Clear);
                }
            }
        }

        let mut map = Map::for_tests(2, terrain);

        assert_eq!(from.distance(&to), 4);

        // All interior samples are clear.
        assert!(visible(&map, from, to, 4));

        // Place a cliff at the last interior sample.
        map.set_terrain_for_tests(Cell { q: 1, r: 0 }, Terrain::Cliff);

        // Visibility must now be blocked.
        assert!(!visible(&map, from, to, 4));
    }

    // Checks that the optimized nearest-cell search matches an exhaustive search.
    #[test]
    fn test_nearest_candidates_brute_force() {
        // Test different denominators, including negative coordinates.
        for n in 1_i64..=6 {
            for q_num in (-2 * n)..=(2 * n) {
                for r_num in (-2 * n)..=(2 * n) {
                    let mut expected = Vec::new();
                    let mut min_distance = i64::MAX;

                    // Exhaustively check cells in a sufficiently large area.
                    for q in -4_i32..=4 {
                        for r in -4_i32..=4 {
                            let cell = Cell { q, r };

                            let distance = squared_distance_to_sample(cell, q_num, r_num, n);

                            if distance < min_distance {
                                min_distance = distance;
                                expected.clear();
                                expected.push(cell);
                            } else if distance == min_distance {
                                expected.push(cell);
                            }
                        }
                    }

                    // Compare with the optimized algorithm.
                    let mut actual = nearest_candidates(q_num, r_num, n);

                    // Sort because candidate order is not important.
                    expected.sort();
                    actual.sort();

                    assert_eq!(
                        actual, expected,
                        "Mismatch for q_num={}, r_num={}, n={}",
                        q_num, r_num, n
                    );
                }
            }
        }
    }
}

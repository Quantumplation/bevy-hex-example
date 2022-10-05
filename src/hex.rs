/// A coordinate on a hex grid, representing distances along the various directions of travel
/// Invariant: In order to represent a valid hex coordinate, q + r + s must equal 0
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HexCoord {
    pub q: isize,
    pub r: isize,
    pub s: isize,
}

// The directions you can move on a hex grid
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    None,
    North,
    South,
    Northeast,
    Southwest,
    Northwest,
    Southeast,
}
impl Direction {
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            None => None,
            North => South,
            South => North,
            Northeast => Southwest,
            Southwest => Northeast,
            Northwest => Southeast,
            Southeast => Northwest,
        }
    }
}

impl HexCoord {
    /// The origin of an infinite hex grid
    pub fn origin() -> Self {
        HexCoord { q: 0, r: 0, s: 0 }
    }
    /// Construct a hex coordinate from two pieces of information, enforcing the invariant on the third
    pub fn new(q: isize, r: isize) -> Self {
        HexCoord { q, r, s: -q - r }
    }

    /// The coordinate to the north
    pub fn north(&self) -> Self {
        Self::new(self.q + 0, self.r - 1)
    }
    /// The coordinate to the south
    pub fn south(&self) -> Self {
        Self::new(self.q + 0, self.r + 1)
    }

    /// The coordinate to the northeast
    pub fn northeast(&self) -> Self {
        Self::new(self.q + 1, self.r - 1)
    }
    /// The coordinate to the southwest
    pub fn southwest(&self) -> Self {
        Self::new(self.q - 1, self.r + 1)
    }

    /// The coordinate to the northwest
    pub fn northwest(&self) -> Self {
        Self::new(self.q - 1, self.r + 0)
    }
    /// The coordinate to the southeast
    pub fn southeast(&self) -> Self {
        Self::new(self.q + 1, self.r + 0)
    }

    /// The coordinate in a specific direction
    pub fn neighbor(&self, dir: Direction) -> Self {
        use Direction::*;
        match dir {
            None => self.clone(),
            North => self.north(),
            South => self.south(),
            Northeast => self.northeast(),
            Southwest => self.southwest(),
            Northwest => self.northwest(),
            Southeast => self.southeast(),
        }
    }

    /// Yield the neighbor coordinates, starting from North and going clockwise
    pub fn neighbors<'a>(&'a self) -> impl Iterator<Item = HexCoord> + 'a {
        struct NeighborIter<'a> {
            c: &'a HexCoord,
            iter: std::slice::Iter<'a, Direction>,
        }
        impl<'a> Iterator for NeighborIter<'a> {
            type Item = HexCoord;
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next().map(|d| self.c.neighbor(d.clone()))
            }
        }
        NeighborIter {
            c: self,
            iter: DIRECTIONS.iter(),
        }
    }
}

/// All directions, for convenient enumeration
pub const DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::Northeast,
    Direction::Southeast,
    Direction::South,
    Direction::Southwest,
    Direction::Northwest,
];

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn zero_sum_invariant() {
        let mut current = HexCoord::origin();
        let mut rand = rand::thread_rng();
        for _ in 0..1000 {
            let dir = DIRECTIONS[rand.gen_range(0..DIRECTIONS.len())].clone();
            current = current.neighbor(dir);
        }
    }

    #[test]
    fn enumerate_neighbors() {
        let mut rand = rand::thread_rng();
        let (q, r) = (rand.gen_range(-100..100), rand.gen_range(-100..100));
        let point = HexCoord::new(q, r);
        let neighbors: Vec<_> = point.neighbors().collect();
        assert_eq!(6, neighbors.len());
        for (neighbor, dir) in neighbors.iter().zip(DIRECTIONS) {
            assert_eq!(point, neighbor.neighbor(dir.opposite()));
        }
    }
}

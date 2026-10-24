

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Direction { 
    East, 
    NorthEast, 
    NorthWest, 
    West, 
    SouthWest, 
    SouthEast 
}

impl Direction {
    pub fn offset(self) -> (i32, i32){
        match self{
            Direction::East => (1, 0),
            Direction::NorthEast => (1, -1),
            Direction::NorthWest => (0, -1),
            Direction::West => (-1, 0),
            Direction::SouthWest => (-1, 1),
            Direction::SouthEast => (0, 1),
        }
    }
    
    
    pub fn relative_angle(self, other: Direction) -> u8 {
        let a = self as i32;
        let b = other as i32;

        let diff = (a - b).abs();

        diff.min(6 - diff) as u8
    }

}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Cell{
    pub q: i32,
    pub r: i32
}


impl Cell{
    fn s(self) -> i32 {
        -self.q -self.r 
    }

    pub fn neighbour(self, direction: Direction) -> Cell{
        let offset = direction.offset(); 
        Cell{
            q: self.q + offset.0,
            r: self.r + offset.1,
        }
    }

    pub fn distance (self, other: Cell) -> u32{
        (((self.q - other.q).abs() + (self.r - other.r).abs() + (self.s() - other.s()).abs() )/2) as u32
        //we could use this other implementation (not documented)
        //(((self.q - other.q).abs()).max( (self.r - other.r).abs()).max((self.s() - other.s()).abs())) as u32
    }
}


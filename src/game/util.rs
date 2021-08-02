#[derive(Clone,Copy, Eq, PartialEq,Debug)]
pub struct Size {
    pub x:usize,
    pub y:usize,
}

impl From<[usize;2]> for Size {
    fn from(pos: [usize; 2]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
        }
    }
}

impl From<(usize,usize)> for Size {
    fn from(pos: (usize,usize)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
        }
    }
}

#[derive(Clone,Copy, Eq, PartialEq,Debug)]
pub struct Position {
    pub x:usize,
    pub y:usize,
}

impl From<[usize;2]> for Position {
    fn from(pos: [usize; 2]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
        }
    }
}

impl From<(usize,usize)> for Position {
    fn from(pos: (usize,usize)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
        }
    }
}

impl From<Position> for (usize,usize)  {
    fn from(pos: Position) -> Self {
        return (pos.x,pos.y);
    }
}


#[derive(Clone,Copy, PartialEq, Eq,Debug)]
pub enum Direction {
    Up,Down,Left,Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up    => Self::Down,
            Self::Down  => Self::Up,
            Self::Left  => Self::Right,
            Self::Right => Self::Left,
        } 
    }
}

impl std::ops::Add for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x+rhs.x,
            y: self.y+rhs.y,
        }
    }
}


impl std::ops::Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up    => Position{x:self.x,y:self.y+1},
            Direction::Down  => Position{x:self.x,y:self.y-1},
            Direction::Right => Position{x:self.x+1,y:self.y},
            Direction::Left  => Position{x:self.x-1,y:self.y},
        }
    }
}


// Food
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FoodGroup {
    Grow,
    Shrink,
    Poison,
}

pub struct Food {
    pub group: FoodGroup,
    pub pos:  Position,
}

impl Food {
    pub fn new<T: Into<Position>>(pos: T, group: FoodGroup) -> Self {
        Self {pos:pos.into(),group}
    }
}
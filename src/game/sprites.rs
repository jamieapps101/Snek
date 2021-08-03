use std::collections::{VecDeque,vec_deque::Iter};
use super::util::{Position, Direction, Size, FoodGroup};

// Snake
pub struct Snake {
    segments: VecDeque<Segment>,
    travel_dir: Direction,
    travelled_dir: Direction,
    map_size: Size,
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Segment {
    pub pos: Position,
}

impl From<[usize;2]> for Segment {
    fn from(pos: [usize; 2]) -> Self {
        Self {
            pos: pos.into(),
        }
    }
}

impl Segment {
    fn new<T: Into<Position>>(pos: T) -> Self {
        Self {
            pos: pos.into(),
        }
    }
}

impl Snake {
    pub fn new<T: Into<Position>>(pos: T, map_size: Size) -> Self {
        // println!("--> new");
        let mut segments = VecDeque::new();
        // segments.push_front(Segment::new(pos));
        segments.push_back(Segment::new(pos));
        // println!("segments:\n{:?}",segments);
        Self {
            segments,
            travel_dir: Direction::Up,
            travelled_dir: Direction::Up,
            map_size,
        }
    }

    pub fn set_dir(&mut self, dir: Direction) {
        // this is to stop the player going back into themselves immediately
        if dir != self.travelled_dir.opposite() {
            self.travel_dir = dir;
        }
    }

    pub fn get_next_head_pos(&mut self) -> Position {
        // println!("--> get_next_head_pos");
        // println!("\t 0.self.segments: {:?}",self.segments);
        let (x,y) = self.segments.get(0).unwrap().pos.into();

        let next_pos: Position = match self.travel_dir {
            // this handles mapping onto the opposite side of the map
            Direction::Up if y == (self.map_size.y-1) => {
                [x,0].into()
            },
            Direction::Down if y == 0 => {
                [x,self.map_size.y-1].into()
            },
            Direction::Right if x == (self.map_size.x-1) => {
                [0,y].into()
            },
            Direction::Left if x == 0 => {
                [self.map_size.x-1,y].into()
            },
            // this handles the snake moving normally
            _ => {
                self.segments.get(0).unwrap().pos+self.travel_dir
            }
        };
        // println!("\t 1.self.segments: {:?}",self.segments);
        return next_pos;
    }

    pub fn next_head_pos_in_body(&mut self) -> bool {
        // println!("--> next_head_pos_in_body");
        let next_head_pos = self.get_next_head_pos();
        return self.is_in_snake(next_head_pos);
    }

    pub fn is_in_snake<T: Into<Position>>(&mut self, pos: T) -> bool {
        let local_pos : Position = pos.into();
        for seg in self.segments.iter() {
            if local_pos==seg.pos {
                return true;
            }
        }
        return false;
    }

    fn add_front_segment<T: Into<Segment>>(&mut self, seg: T) {
        self.segments.push_front(seg.into());
    }

    fn drop_last_segment(&mut self) {
        if self.segments.make_contiguous().len() > 1 {
            self.segments.pop_back();
        }
    }

    // use the current travel direction to update the snake position based on the direction.
    pub fn slither(&mut self, food_to_eat: Option<FoodGroup>) -> SnakeState {
        if self.next_head_pos_in_body() {
            return SnakeState::Dead(Reason::Collision);
        }
        // println!("\t 1.self.segments: {:?}",self.segments);
        if let Some(food) = food_to_eat {
            match food {
                FoodGroup::Grow => {
                    // println!("\t 2.self.segments: {:?}",self.segments);
                    let next_pos = self.get_next_head_pos();
                    // println!("\t 3.self.segments: {:?}",self.segments);
                    let seg = Segment::new(next_pos);
                    self.add_front_segment(seg);
                    // println!("\t 4.self.segments: {:?}",self.segments);
                }
                FoodGroup::Poison => {
                    return SnakeState::Dead(Reason::Poison);
                }
                FoodGroup::Shrink => {
                    self.drop_last_segment();
                    self.drop_last_segment();
                    let next_pos = self.get_next_head_pos();
                    let seg = Segment::new(next_pos);
                    self.add_front_segment(seg);
                }
            }
        } else {
            let next_pos = self.get_next_head_pos();
            self.drop_last_segment();
            let seg = Segment::new(next_pos);
            self.add_front_segment(seg);
        }
        self.travelled_dir = self.travel_dir;
        return SnakeState::Alive;
    }

    pub fn iter_segments(&self) -> Iter<Segment> {
        return self.segments.iter();
    }
}


#[derive(PartialEq, Debug)]
pub enum SnakeState {
    Alive,
    Dead(Reason),
}

#[derive(PartialEq, Debug)]
pub enum Reason {
    Collision,
    Poison,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn init_snake() {
        let s = Snake::new([2,2], Size{x:5,y:5});

        assert_eq!(s.segments.len(),1);
        assert_eq!(s.travel_dir, Direction::Up);
        assert_eq!(s.travelled_dir, Direction::Up);
        assert_eq!(s.map_size, Size{x:5,y:5});
    }

    #[test]
    fn change_snake_dir_orthogonal() {
        let mut s = Snake::new([2,2], Size{x:5,y:5});
        
        assert_eq!(s.travel_dir, Direction::Up);
        
        s.set_dir(Direction::Left);

        assert_eq!(s.segments.len(),1);
        assert_eq!(s.travel_dir, Direction::Left);
        assert_eq!(s.travelled_dir, Direction::Up);
        assert_eq!(s.map_size, Size{x:5,y:5});
    }

    #[test]
    fn change_snake_dir_opposite() {
        let mut s = Snake::new([2,2], Size{x:5,y:5});
        
        assert_eq!(s.travel_dir, Direction::Up);
        
        s.set_dir(Direction::Down);

        assert_eq!(s.segments.len(),1);
        assert_eq!(s.travel_dir, Direction::Up);
        assert_eq!(s.travelled_dir, Direction::Up);
        assert_eq!(s.map_size, Size{x:5,y:5});
    }

    #[test]
    fn get_next_head_pos_in_bounds() {
        let mut s = Snake::new([2,2], Size{x:5,y:5});
        s.set_dir(Direction::Up);
        assert_eq!(s.get_next_head_pos(),[2,3].into());
        s.set_dir(Direction::Right);
        assert_eq!(s.get_next_head_pos(),[3,2].into());
        s.set_dir(Direction::Left);
        assert_eq!(s.get_next_head_pos(),[1,2].into());

        s.travelled_dir = Direction::Down;
        s.set_dir(Direction::Down);
        assert_eq!(s.get_next_head_pos(),[2,1].into());
    }

    #[test]
    fn get_next_head_pos_out_of_bounds() {
        let mut s = Snake::new([4,4], Size{x:5,y:5});
        s.set_dir(Direction::Up);
        assert_eq!(s.get_next_head_pos(),[4,0].into());
        s.set_dir(Direction::Right);
        assert_eq!(s.get_next_head_pos(),[0,4].into());

        let mut s = Snake::new([0,0], Size{x:5,y:5});
        s.set_dir(Direction::Left);
        assert_eq!(s.get_next_head_pos(),[4,0].into());

        s.travelled_dir = Direction::Down;
        s.set_dir(Direction::Down);
        assert_eq!(s.get_next_head_pos(),[0,4].into());
    }

    #[test]
    fn head_in_body_detection_false() {
        let mut s = Snake::new([4,4], Size{x:5,y:5});
        assert_eq!(s.next_head_pos_in_body(),false);
    }

    #[test]
    fn add_segment() {
        let mut s = Snake::new([4,4], Size{x:5,y:5});
        s.add_front_segment([4,3]);
        s.add_front_segment([3,3]);

        
        let seg_vec : Vec<Segment> = vec![
            [4,4].into(),
            [4,3].into(),
            [3,3].into()
        ];
        let ref_seg : VecDeque<Segment> = VecDeque::from(seg_vec);

        assert_eq!(s.segments.len(),3);
        s.segments.iter().rev().zip(ref_seg.iter()).for_each(|(s,ref_s)|{
            assert_eq!(s,ref_s);
        });
    }

    #[test]
    fn test_iter_segments() {
        let mut s = Snake::new([4,4], Size{x:5,y:5});
        s.add_front_segment([4,3]);
        s.add_front_segment([3,3]);

        
        let seg_vec : Vec<Segment> = vec![
            [4,4].into(),
            [4,3].into(),
            [3,3].into()
        ];
        let ref_seg : VecDeque<Segment> = VecDeque::from(seg_vec);

        assert_eq!(s.segments.len(),3);
        s.iter_segments().rev().zip(ref_seg.iter()).for_each(|(s,ref_s)|{
            assert_eq!(s,ref_s);
        });
    }

    #[test]
    fn drop_segment() {
        let mut s = Snake::new([4,4], Size{x:5,y:5});
        s.add_front_segment([4,3]);
        s.add_front_segment([3,3]);

        s.drop_last_segment();

        let seg_vec = vec![
            [4,3].into(),
            [3,3].into()
        ];
        let ref_seg : VecDeque<Segment> = VecDeque::from(seg_vec);

        assert_eq!(s.segments.len(),2);

        s.segments.iter().rev().zip(ref_seg.iter()).for_each(|(s,ref_s)|{
            assert_eq!(s,ref_s);
        });


    }

    #[test]
    fn head_in_body_detection_true() {
        let mut s = Snake::new([4,4], Size{x:5,y:5});
        s.add_front_segment([4,3]);
        s.add_front_segment([4,2]);
        s.add_front_segment([4,1]);
        s.add_front_segment([3,1]);
        s.add_front_segment([2,1]);
        s.add_front_segment([2,2]);
        s.add_front_segment([2,3]);
        s.add_front_segment([3,3]);
        s.set_dir(Direction::Right);
        assert_eq!(s.get_next_head_pos(), [4,3].into());
        assert_eq!(s.next_head_pos_in_body(),true);
    }

    #[test]
    fn slither_test_no_food() {
        let mut s = Snake::new([3,3], Size{x:5,y:5});
        s.add_front_segment([2,3]);
        s.add_front_segment([2,2]);
        s.add_front_segment([2,1]);
        s.travel_dir    = Direction::Left;
        s.travelled_dir = Direction::Left;
        
        assert_eq!(s.slither(None),SnakeState::Alive);
        let seg_vec = vec![
            [2,3].into(),
            [2,2].into(),
            [2,1].into(),
            [1,1].into()
        ];
        let ref_seg : VecDeque<Segment> = VecDeque::from(seg_vec);
        assert_eq!(s.segments.len(),4);
        s.iter_segments().zip(ref_seg.iter().rev()).for_each(|(s,ref_s)|{
            assert_eq!(s,ref_s);
        });

        s.set_dir(Direction::Up);
        assert_eq!(s.slither(None),SnakeState::Alive);
        let seg_vec = vec![
            [2,2].into(),
            [2,1].into(),
            [1,1].into(),
            [1,2].into()
        ];
        let ref_seg : VecDeque<Segment> = VecDeque::from(seg_vec);
        assert_eq!(s.segments.len(),4);
        s.iter_segments().zip(ref_seg.iter().rev()).for_each(|(s,ref_s)|{
            assert_eq!(s,ref_s);
        });
    }

    #[test]
    fn slither_test_no_food_one_seg() {
        let mut s = Snake::new([3,3], Size{x:5,y:5});
        
        assert_eq!(s.slither(None),SnakeState::Alive);
        let seg_vec = vec![
            [3,4].into(),
        ];
        let ref_seg : VecDeque<Segment> = VecDeque::from(seg_vec);
        assert_eq!(s.segments.len(),1);
        s.iter_segments().zip(ref_seg.iter().rev()).for_each(|(s,ref_s)|{
            assert_eq!(s,ref_s);
        });
    }

    #[test]
    fn slither_test_grow_food() {
        let mut s = Snake::new([3,3], Size{x:5,y:5});
        s.add_front_segment([2,3]);
        s.add_front_segment([2,2]);
        s.add_front_segment([2,1]);
        s.travel_dir    = Direction::Left;
        s.travelled_dir = Direction::Left;
        
        assert_eq!(s.slither(Some(FoodGroup::Grow)),SnakeState::Alive);
        let seg_vec = vec![
            [3,3].into(),
            [2,3].into(),
            [2,2].into(),
            [2,1].into(),
            [1,1].into()
        ];
        let ref_seg : VecDeque<Segment> = VecDeque::from(seg_vec);
        assert_eq!(s.segments.len(),5);
        s.iter_segments().zip(ref_seg.iter().rev()).for_each(|(s,ref_s)|{
            assert_eq!(s,ref_s);
        });

        s.set_dir(Direction::Up);
        assert_eq!(s.slither(None),SnakeState::Alive);
        let seg_vec = vec![
            [2,3].into(),
            [2,2].into(),
            [2,1].into(),
            [1,1].into(),
            [1,2].into()
        ];
        let ref_seg : VecDeque<Segment> = VecDeque::from(seg_vec);
        assert_eq!(s.segments.len(),5);
        s.iter_segments().zip(ref_seg.iter().rev()).for_each(|(s,ref_s)|{
            assert_eq!(s,ref_s);
        });
    }

    #[test]
    fn slither_test_poison_food() {
        let mut s = Snake::new([3,3], Size{x:5,y:5});
        s.add_front_segment([2,3]);
        s.add_front_segment([2,2]);
        s.add_front_segment([2,1]);
        s.travel_dir    = Direction::Left;
        s.travelled_dir = Direction::Left;
        
        assert_eq!(s.slither(Some(FoodGroup::Poison)),SnakeState::Dead(Reason::Poison));
    }

    #[test]
    fn test_in_snake() {
        let mut s = Snake::new([3,3], Size{x:5,y:5});
        s.add_front_segment([2,3]);
        s.add_front_segment([2,2]);
        s.add_front_segment([2,1]);

        assert_eq!(s.is_in_snake([4,5]),false);
        assert_eq!(s.is_in_snake([0,0]),false);
        assert_eq!(s.is_in_snake([2,0]),false);

        assert_eq!(s.is_in_snake([3,3]),true);
        assert_eq!(s.is_in_snake([2,3]),true);
        assert_eq!(s.is_in_snake([2,2]),true);
        assert_eq!(s.is_in_snake([2,1]),true);
    }
}
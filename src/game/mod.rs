mod sprites;
mod util;

use sprites::{Snake};
pub use sprites::{SnakeState,Reason};
use util::{Size, Direction, Position, Food};
pub use util::FoodGroup;

pub struct GameState {
    snake: Snake,
    foods: Vec<util::Food>,
    map_size: Size,
    score: usize,
}

#[derive(PartialEq)]
pub enum SnakeControl {
    Up,Down,Left,Right,None,
}

impl From<SnakeControl> for Direction {
    fn from(sc: SnakeControl) -> Self {
        match sc {
            SnakeControl::Up => Direction::Up,
            SnakeControl::Down => Direction::Down,
            SnakeControl::Left => Direction::Left,
            SnakeControl::Right => Direction::Right,
            SnakeControl::None => unreachable!(),
        }
    }
}


impl GameState {
    pub fn new<P: Into<Position>,S: Into<Size>+Clone>(map_size: S, snake_position: P) -> Self {
        // let map_x = map_size.x as f32;
        // let map_y = map_size.y as f32;
        // let snake_x = (map_x*0.5).floor() as usize;
        // let snake_y = (map_y*0.5).floor() as usize;
        let pos = snake_position.into();
        let snake_x = pos.x;
        let snake_y = pos.y;
        Self {
            snake: Snake::new([snake_x,snake_y], map_size.clone().into()),
            foods: Vec::new(),
            map_size: map_size.into(),
            score: 0,
        }
    }

    pub fn update(&mut self,control: SnakeControl, gen_new_food: bool) -> SnakeState {
        // apply control input to snake
        if control != SnakeControl::None {
            self.snake.set_dir(control.into());
        }

        //// get next head pos
        let next_head_pos = self.snake.get_next_head_pos();
        //// check if theres any food
        let mut food_to_eat: Option<FoodGroup> = None;
        for index in 0..self.foods.len() {
            if self.foods[index].pos == next_head_pos {
                let food = self.foods.remove(index);
                food_to_eat = Some(food.group);
                self.score += 1;

                break;
            }
        }
        // println!("food to eat: {:?}",food_to_eat);
        //// let the snake slither and give it any food it finds
        let snake_state = self.snake.slither(food_to_eat);
        if gen_new_food {
            self.gen_food();
        }

        return snake_state;
    }

    pub fn gen_food(&mut self) {
        // todo make this random
        let food_pos : Position = [4,3].into();

        if !self.snake.is_in_snake(food_pos) {
            self.add_food(Food::new(food_pos, FoodGroup::Grow));
        }
    }

    fn add_food(&mut self, new_food: Food) {
        let mut present = false;
        self.foods.iter().for_each(|f| {
            if *f==new_food {
                present = true;
            }
        });
        if !present {
            self.foods.push(new_food)
        }
    }

    pub fn get_render_map(&self) -> RenderMap {
        let mut map = vec![vec![Item::Nothing; self.map_size.x];self.map_size.y];

        // fill in the snake
        self.snake.iter_segments().enumerate().for_each(|(index,seg)| {
            if index==0 {
                map[seg.pos.x][seg.pos.y] = Item::SnakeHead;
            } else {
                map[seg.pos.x][seg.pos.y] = Item::Snake;
            }
        });

        // fill in the food
        self.foods.iter().for_each(|food| {
            map[food.pos.x][food.pos.y] = Item::Food(food.group);
        });

        return map;
    }
}

pub type RenderMap = Vec<Vec<Item>>;

#[derive(Clone, Copy)]
pub enum Item {
    Snake,
    SnakeHead,
    Food(FoodGroup),
    Nothing,
}


#[cfg(test)] 
mod test {
    use super::*;
    #[test]
    fn init_gs() {
        let gs = GameState::new([10,10],[4,4]);
        assert_eq!(gs.foods,     Vec::new());
        assert_eq!(gs.map_size,  Size{x:10,y:10});
        assert_eq!(gs.score,     0);
    }

    #[test]
    fn run_update_10() {
        let mut gs = GameState::new([10,10],[4,4]);
        for _i in 0..10 {
            gs.update(SnakeControl::None, false);
        }
        assert_eq!(gs.snake.get_next_head_pos(), [4,5].into());
    }

    #[test]
    fn run_update_10_with_init_food() {
        let mut gs = GameState::new([10,10],[4,4]);
        let food_count_ref = [1,1,1,1,1,1,1,1,0,0];
        gs.gen_food();
        for i in 0..10 {
            gs.update(SnakeControl::None, false);
            assert_eq!(food_count_ref[i], gs.foods.len());
        }
        assert_eq!(gs.snake.get_next_head_pos(), [4,5].into());
    }

    #[test]
    fn run_update_10_with_const_food() {
        let mut gs = GameState::new([10,10],[4,4]);
        // let food_count_ref = [1,1,1,1,1,1,1,1,0,0];
        gs.gen_food();
        for _i in 0..10 {
            gs.update(SnakeControl::None, true);
            // assert_eq!(food_count_ref[i], gs.foods.len());
        }
        // assert_eq!(gs.snake.get_next_head_pos(), [4,5].into());
    }

}
mod sprites;
mod util;

use sprites::{Snake};
pub use sprites::{SnakeState,Reason};
use util::{Size, Direction};
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
    pub fn new(map_size: Size) -> Self {
        let map_x = map_size.x as f32;
        let map_y = map_size.y as f32;
        let snake_x = (map_x*0.5).floor() as usize;
        let snake_y = (map_y*0.5).floor() as usize;
        Self {
            snake: Snake::new([snake_x,snake_y], map_size),
            foods: Vec::new(),
            map_size,
            score: 0,
        }
    }

    pub fn update(&mut self,control: SnakeControl) -> SnakeState {
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
        return self.snake.slither(food_to_eat)
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

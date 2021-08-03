use std::process::{exit};

use tokio::time::{sleep, Duration};

mod game;
mod ui;
use game::{GameState,SnakeState, SnakeControl};

use ui::UI;


#[tokio::main]
pub async fn main() {

    let mut gs = GameState::new([10,10],[4,4]);
    gs.gen_food();
    let mut ui = UI::new().unwrap();
    ui.clear();
    for _i in 0..40 {
        // println!("i={:?}",i);
        //     // drive snake
        //     let control = ui.get_snake_control();
        let control = SnakeControl::None;
        let gs_update_response = gs.update(control, true);
            if let SnakeState::Dead(reason) = gs_update_response {
                panic!("dead: {:?}",reason);
            }
        
        //     // update ui
            let map = gs.get_render_map();
            ui.render(map);
        
        //     // wait a while
            sleep(Duration::from_millis(500)).await;
    }
    ui.clear();

    exit(0);

}
use std::process::{exit};

use tokio::time::{sleep, Duration};

mod game;
mod ui;
use game::{GameState,SnakeState};

use ui::{UI,UIControl};


#[tokio::main]
pub async fn main() {

    let mut gs = GameState::new([10,10],[4,4]);
    gs.gen_food();
    let mut ui = UI::new().unwrap();
    ui.clear();
    loop {
        let control = ui.get_control();
        if control==UIControl::ExitProgram {
            break;
        }
        let gs_update_response = gs.update(control.get_snake_control(), true);
        if let SnakeState::Dead(reason) = gs_update_response {
            panic!("dead: {:?}",reason);
        }
        
        // update ui
        let map = gs.get_render_map();
        ui.render(map);
        
        //     // wait a while
        sleep(Duration::from_millis(500)).await;
    }
    ui.clear();

    exit(0);

}
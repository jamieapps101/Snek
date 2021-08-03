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
            ui.clear();
            break;
        }
        // update the game state base on user input
        // render out the game for the ui
        let (ss,rd) = gs.update_and_render(control.get_snake_control(), true);
        if let SnakeState::Dead(reason) = ss {
            ui.clear();
            println!("You died: {:?}",reason);
            break;
        }
        // update ui
        ui.render(rd);
        // wait a while
        sleep(Duration::from_millis(500)).await;
    }
    
    exit(0);
}
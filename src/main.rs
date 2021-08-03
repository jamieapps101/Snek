use std::process::{exit};

use clap::{Arg, App};
use tokio::time::{sleep, Duration};

mod game;
use game::{GameState,SnakeState};

mod ui;
use ui::{UI,UIControl};

mod util;
use util::*;

#[tokio::main]
pub async fn main() {
    // Parse the Args
    let matches = App::new("Snek")
                          .version("1.0")
                          .about("CLI Snake Game")
                          .arg(Arg::with_name("dims")
                               .short("d")
                               .long("dims")
                               .value_name("H,W")
                               .help("Sets the game width and height")
                               .takes_value(true))
                          .get_matches();

    let dim_string = matches.value_of("dims").unwrap_or("10,10");
    let dims : [usize; 2] = args_to_dims(dim_string).unwrap();

    // Start the Game
    let mut gs = GameState::new(dims,[0,0]);
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
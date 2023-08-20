use clap::Parser;
use sneko_engine::Game;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
mod sneko_engine;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        println!("Application error: {e}");
        std::process::exit(1);
    }

}

fn run(cli:Cli)-> Result<(),Box<dyn Error>>{
    let game_config=sneko_engine::Config::new(cli.width,cli.heigth,cli.fruit_count);
    let mut game = sneko_engine::Game::new(game_config);
    game.add_player("Hejdula".to_string(),"soket".to_string());
    game.add_player("2".to_string(),"soket".to_string());
    game.add_player("3".to_string(),"soket".to_string());
    game.add_player("4".to_string(),"soket".to_string());
    game.add_player("5".to_string(),"soket".to_string());
    game.start();
    (1..2).for_each(|_|{game.tick();println!("");sleep(Duration::from_millis(400))});
    return Ok(());
}

fn game_loop(game:Game){
    
}


#[derive(Parser)]
#[command(name = "Sneko", author = "Hejdula", version,about = "Does snakey things")]
struct Cli {
    #[arg(short='x', long, default_value_t=10)]
    width: usize,
    #[arg(short='y', long, default_value_t=10)]
    heigth: usize,
    #[arg(short='f', long, default_value_t=1)]
    fruit_count:usize,
}

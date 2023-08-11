use clap::Parser;
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
    let game = sneko_engine::Game::new(game_config);
    game.add_player("Hejdula".to_string(),"soket".to_string());
    (1..13).for_each(|_|{game.tick();println!("");sleep(Duration::from_secs(1))});
    return Ok(());
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

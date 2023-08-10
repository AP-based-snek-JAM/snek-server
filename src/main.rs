use clap::Parser;
use std::error::Error;
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
    game.print_field();
    game.tick();
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

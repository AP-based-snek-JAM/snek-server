use clap::Parser;

mod game_engine {
    pub struct GameField {
        pub data: Box<[Box<[i32]>]>
    }
    impl GameField{
        pub fn new(width:usize,heigth:usize) -> GameField {
            GameField {
                data: vec![
                    vec![0;width].into_boxed_slice();heigth
                ].into_boxed_slice() 
            }
        }
        pub fn print_field(&self){
            self.data.iter().for_each(|row|println!("{:?}",row));
        }
    }
}


fn main() {
    let config = Cli::parse();
    let game_field = game_engine::GameField::new(config.width,config.heigth);
    game_field.print_field();
}





#[derive(Parser)]
#[command(name = "Sneko", author = "Hejdula", version = "1.0",about = "Does snakey things")]
struct Cli {
    #[arg(short='x', long, default_value_t=10)]
    width: usize,
    #[arg(short='y', long, default_value_t=10)]
    heigth: usize,
}

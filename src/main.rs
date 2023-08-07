use clap::Parser;

mod sneko_engine {
    pub struct GameField {
        pub data: Box<[Box<[i32]>]>,
        fruit_count:usize
    }
    impl GameField{
        pub fn new(width:usize,heigth:usize,fruit_count:usize) -> GameField {
            GameField {
                data: vec![
                    vec![0;width].into_boxed_slice();heigth
                ].into_boxed_slice(), 
                fruit_count:1,
            }
        }
        pub fn print_field(&self){
            self.data.iter().for_each(|row|println!("{:?}",row));
        }
        pub fn slither(){
        }
    }
    pub enum Direction{
        Up,
        Down,
        Left,
        Right
    }
    pub struct Sneko {
        pub coordinates: (usize,usize)
    }
}


fn main() {
    let config = Cli::parse();
    let game_field = sneko_engine::GameField::new(config.width,config.heigth,config.fruit_count);
    game_field.print_field();
}





#[derive(Parser)]
#[command(name = "Sneko", author = "Hejdula", version = "1.0",about = "Does snakey things")]
struct Cli {
    #[arg(short='x', long, default_value_t=10)]
    width: usize,
    #[arg(short='y', long, default_value_t=10)]
    heigth: usize,
    #[arg(short='f', long, default_value_t=1)]
    fruit_count:usize,
}

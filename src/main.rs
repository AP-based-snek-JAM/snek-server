use clap::Parser;
mod sneko_engine {
    use std::cell::RefCell;
    pub struct Game {
        field: RefCell<Box<[Box<[isize]>]>>,
        fruit_count:usize,
        snekos:Vec<Sneko>
    }
    impl Game{
        pub fn new(width:usize,heigth:usize,fruit_count:usize) -> Game {
            Game {
                field: RefCell::new(vec![
                    vec![0;width].into_boxed_slice();heigth
                ].into_boxed_slice()), 
                fruit_count:1,
                snekos:vec![]
            }
        }
        pub fn print_field(&self){
            self.field.borrow().iter().for_each(|row|println!("{:?}",row));
        }
        pub fn tick(&mut self){
            for row in self.field.borrow_mut().iter_mut(){
                for value in row.iter_mut(){
                    if *value != 0 {
                        *value -= 1;
                    }
                }     
            }
//            self.field.iter().for_each(|row|{
//                row.iter().for_each(|&value|{
//                    if value!=0{return value-=1}
//                })
//            })
//            self.field.iter()
//                .map(|row|row.iter()
//                     .map(|value|{
//                         if *value!=0{return *value-=1} else {value}
//                     }).collect::<&Box<i32>>());
        }
        fn spawn_fruit(){
            //nomnom
        }
    }
    pub enum Direction{
        Up,
        Down,
        Left,
        Right
    }
    pub struct Sneko {
        pub coordinates: (usize,usize),
        pub direction:Direction
    }
    impl Sneko {
        pub fn new(coordinates:(usize,usize))-> Sneko{
            Sneko { coordinates, direction: (Direction::Right) }    
        }
        pub fn slither(){
            //wiggly wiggle
        }
    }
}


fn main() {
    let config = Cli::parse();
    let game = sneko_engine::Game::new(config.width,config.heigth,config.fruit_count);
    game.print_field();
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

use std::cell::RefCell;
pub struct Config {
    pub field_width:usize,
    pub field_heigth:usize,
    pub fruit_count:usize,
    
}
impl Config {
    pub fn new(field_width:usize,field_heigth:usize,fruit_count:usize) -> Config {
        Config{field_width,field_heigth,fruit_count}
    }
}
pub struct Game {
    soket: String,
    field: RefCell<Box<[Box<[usize]>]>>,
    config: Config,
    snekos:Vec<Sneko>
}
impl Game{
    pub fn new(config:Config) -> Game {
        Game {
            soket: String::new(),
            field: RefCell::new(vec![
                vec![0;config.field_width].into_boxed_slice();config.field_heigth
            ].into_boxed_slice()), 
            config,
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
    player_soket: String,
    coordinates: (usize,usize),
    direction:Direction,
    length:usize
}
impl Sneko {
    pub fn new(player_soket:String,coordinates:(usize,usize),length:usize)-> Sneko{
        Sneko { player_soket,coordinates, direction: (Direction::Right),length }    
    }
    pub fn slither(&self){

        //wiggly wiggle
    }
}


use std::cell::RefCell;
use std::rc::{ Weak,Rc };

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
    snekos:RefCell<Vec<Sneko>>,
    fruits:RefCell<Vec<Coords>>
}
impl Game{
    pub fn new(config:Config) -> Game {
        Game {
            soket: String::new(),
            field: RefCell::new(vec![
                vec![0;config.field_width].into_boxed_slice();config.field_heigth
            ].into_boxed_slice()), 
            config,
            snekos:RefCell::new(vec![]),
            fruits:RefCell::new(vec![])
        }
    }
    pub fn print_field(&self){
        self.field.borrow().iter().for_each(|row|println!("{:?}",row));
    }
    pub fn tick(&self){
        for row in self.field.borrow_mut().iter_mut(){
            for value in row.iter_mut(){
                if *value != 0 {
                    *value -= 1;
                }
            }
        }
        self.snekos.borrow().iter().for_each(|snek|{ 
            if *snek.alive.borrow() == false {return}
            snek.slither();
            let snek_coords = dbg!(snek.coords.borrow());
            if snek_coords.x>=self.config.field_width || snek_coords.y>=self.config.field_heigth {  
            }
        })
    }
    fn spawn_fruit(){
        //nomnom
    }
    pub fn add_player(&self,nickname:String,player_soket:String){
        self.snekos.borrow_mut().push(Sneko::new(nickname,player_soket))
    }
}

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}
#[derive(Debug)]
pub struct Coords {
    pub x:usize,
    pub y:usize,
}

pub struct Sneko {
    nickname:String,
    player_soket: String,
    pub coords:RefCell<Coords>,
    pub direction:RefCell<Direction>,
    length:usize,
    pub alive:RefCell<bool>
}

impl Sneko {
    pub fn new(nickname:String,player_soket:String)-> Sneko{
        Sneko {nickname,player_soket,coords:RefCell::new(Coords{x:0,y:0}), direction: RefCell::new(Direction::Right),length:1,alive:RefCell::new(true)}    
    }
    pub fn slither(&self){
        match *self.direction.borrow() {
            Direction::Up => {self.coords.borrow_mut().y+=1},
            Direction::Down =>{  
                let mut coords_borrow = self.coords.borrow_mut(); 
                if coords_borrow.y > 0 {coords_borrow.y-=1} 
            },
            Direction::Left =>{ 
                let mut coords_borrow = self.coords.borrow_mut(); 
                if coords_borrow.x > 0 {coords_borrow.x-=1} 
            },
            Direction::Right =>{ 
                self.coords.borrow_mut().x+=1 
            },
        }
        //wiggly wiggle
    }
}


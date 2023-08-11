use std::cell::RefCell;
use rand::Rng;
//use std::rc::{ Weak,Rc };

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
pub enum Point {
    Empty,
    Sneko(usize),
    Fruit,
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
        self.update_field_values();
        self.decrease_field_values();
        self.slither_alive_snekos();
        self.update_field_values();
    }
    fn number_of_alive_snekos(&self) -> usize{
        let mut result:usize = 0;
        self.snekos.borrow().iter().for_each(|snek|if let true = *snek.alive.borrow(){result+=1});
        result
    }
    fn update_field_values(&self){ 
        self.snekos.borrow().iter().for_each(|snek|{
            match *snek.alive.borrow(){
                true => {
                    let snek_coords = snek.coords.borrow_mut();
                    self.field.borrow_mut()[snek_coords.y][snek_coords.x] = *snek.length.borrow();
                }
                false => {}
        }})  
    }
    fn decrease_field_values(&self){
        self.field.borrow_mut().iter_mut().for_each(|row|{
            row.iter_mut().for_each(|value|{
                if *value != 0 {
                    *value -= 1;
                }
            })
        });
    }
    fn slither_alive_snekos(&self){    
        self.snekos.borrow().iter().for_each(|snek|{ 
            if *snek.alive.borrow() == false {return}
            snek.slither();
            let snek_coords = snek.coords.borrow();
            if snek_coords.x>=self.config.field_width || snek_coords.y>=self.config.field_heigth || self.field.borrow()[snek_coords.y][snek_coords.x] > 0 {    
                *snek.alive.borrow_mut() = false;
            }
        })
    }
    fn spawn_fruits(&self){
        while self.fruits.borrow().len() < self.config.fruit_count{
            let mut rnd = rand::thread_rng();
            let rand_coords = Coords{
                x: rnd.gen_range(0..self.config.field_width),
                y: rnd.gen_range(0..self.config.field_heigth)
            };
            if self.field.borrow()[rand_coords.y][rand_coords.x]==0{
                self.fruits.borrow_mut().push(rand_coords)
            }
        }
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
    pub coords: RefCell<Coords>,
    pub direction: RefCell<Direction>,
    pub body_cells: RefCell<Vec<Coords>>,
    pub length: RefCell<usize>,
    pub alive: RefCell<bool>
}

impl Sneko {
    pub fn new(nickname:String,player_soket:String)-> Sneko{
        Sneko {
            nickname,
            player_soket,
            coords:RefCell::new(Coords{x:0,y:0}), 
            direction: RefCell::new(Direction::Right),
            body_cells: RefCell::new(vec![]),//yet to implement
            length: RefCell::new(3),
            alive:RefCell::new(true)}    
    }
    pub fn slither(&self){
        match *self.direction.borrow() {
            Direction::Up => {self.coords.borrow_mut().y+=1},
            Direction::Down =>{  
                let mut coords_borrow = self.coords.borrow_mut(); 
                match self.coords.borrow().y {
                    0 => {*self.alive.borrow_mut() = false}
                    _ => {coords_borrow.y-=1}
                } },
            Direction::Left =>{ 
                let mut coords_borrow = self.coords.borrow_mut(); 
                match coords_borrow.x {
                    0 => {*self.alive.borrow_mut() = false}
                    _ => {coords_borrow.x-=1}
                }}
            Direction::Right =>{self.coords.borrow_mut().x+=1},
        }
        //wiggly wiggle
    }
}


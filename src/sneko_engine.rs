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
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Point {
    Empty,
    Sneko(usize),
    Fruit,
}
pub struct Game {
    soket: String,
    pub field: Box<[Box<[Point]>]>,
    config: Config,
    snekos:Vec<Sneko>,
    fruits:Vec<Coords>
}
impl Game{
    pub fn new(config:Config) -> Game {
        Game {
            soket: String::new(),
            field: vec![
                vec![Point::Empty;config.field_width].into_boxed_slice();config.field_heigth
            ].into_boxed_slice(), 
            config,
            snekos:vec![],
            fruits:vec![]
        }
    }
    pub fn print_field(&self){
        self.field.iter().for_each(|row|{ 
            println!("");
            row.iter().for_each(|point|{
                match point {
                    Point::Empty=>{print!("░")} //⎕
                    Point::Fruit=>{print!("█")}
                    Point::Sneko(value)=>{print!("▓")}//▩
                }
            })
        })
    }
    pub fn start(&mut self){
        self.spawn_snekos();
        //self.spawn_fruits();
        self.update_field_values();
    }
    pub fn spawn_snekos(&mut self){
        let number_of_snekos = self.snekos.len();
        self.snekos.iter_mut().enumerate().for_each(|(i,snek)|{
            snek.alive=true;
            let side_direction = match i%4{
                0 => {Direction::Left}
                1 => {Direction::Right}
                2 => {Direction::Up}
                3 => {Direction::Down}
                _ => unreachable!()
            };
            snek.prev_direction = side_direction.opposite();
            snek.direction = side_direction.opposite();
            let snekos_on_side = (number_of_snekos/4) + {
                match i%4 <= number_of_snekos%4 {
                    true => {1}
                    false => {0}
                }
            };
            let nth_on_side=1+(i/4);
            let spacing = self.config.field_width as f32 / ((snekos_on_side+1) as f32);
            snek.coords = {match side_direction {
                Direction::Up => {
                    Coords{x:(nth_on_side as f32 * spacing).ceil() as usize,y:self.config.field_heigth-1}}
                Direction::Down => {
                    Coords{x:(nth_on_side as f32 * spacing).ceil() as usize,y:0}}
                Direction::Left => {
                    Coords{x:0,y:(nth_on_side as f32 * spacing).ceil() as usize}}
                Direction::Right => {
                    Coords{x:self.config.field_width-1,y:(nth_on_side as f32 * spacing).ceil() as usize}}
            }};
            println!("{:?},snekos on side:{},nth_on_side:{},{},{:?}",side_direction,snekos_on_side,nth_on_side,spacing,snek.coords)
        });
    }
        
    fn update_field_values(&mut self){ 
        self.snekos.iter().for_each(|snek|{
            if let true = snek.alive{
                self.field[snek.coords.y][snek.coords.x] = Point::Sneko(snek.length);    
            }
       })
    }

    pub fn change_player_direction(&mut self,nickname:String,direction:Direction){
        //let reference_to_sneko: Option<&Sneko> = self.snekos.iter().find(|snek| snek.nickname == nickname);
        //if let Some(snek) = self.snekos.iter().find(|snek| snek.nickname == nickname){
        //    if snek.prev_direction.opposite() != direction{
        //        snek.direction = direction;
        //    }
        //}
    }
    pub fn tick(&mut self){
        if self.number_of_alive_snekos() == 0 {
            println!("GAME OVER");
            return
        }
        //temporary
        
        //temporary
        self.decrease_field_values();
        self.slither_alive_snekos();
        self.handle_movement();
        print! ("\x1B[2J\x1B[1;1H");
        self.print_field();
    }
    fn number_of_alive_snekos(&self) -> usize{
        let mut result:usize = 0;
        self.snekos.iter().for_each(|snek|if let true = snek.alive{result+=1});
        result
    }
    fn handle_movement(&mut self){ 
        self.snekos.iter_mut().for_each(|sneko|{
            let mut dies = false;
            match sneko.alive{
                true => {
                    let point = &mut self.field[sneko.coords.y][sneko.coords.x];
                    match point{
                        Point::Empty=>{
                            *point = Point::Sneko(sneko.length)}
                        Point::Fruit => {
                            *point = Point::Sneko(sneko.length)}
                        Point::Sneko(_) => {dies = true;}
                    }
                }
                false => {}
            }
            if dies {sneko.alive = false}
        })  
    }
    fn decrease_field_values(&mut self){
        self.field.iter_mut().for_each(|row|{
            row.iter_mut().for_each(|point|{
                if let Point::Sneko(value) = point {
                    match value {
                        0|1 => {*point = Point::Empty}
                        x => {*x -= 1}
                    } 
                }
            })
        })
    }
    fn slither_alive_snekos(&mut self){    
        self.snekos.iter_mut().for_each(|snek|{ 
            if snek.alive == false {return}
            snek.slither();
            if snek.coords.x>=self.config.field_width || snek.coords.y>=self.config.field_heigth{    
                snek.alive = false;
            }
        })
    }
    fn spawn_fruits(&mut self){
        while self.fruits.len() < self.config.fruit_count{
            let mut rnd = rand::thread_rng();
            let rand_coords = Coords{
                x: rnd.gen_range(0..self.config.field_width),
                y: rnd.gen_range(0..self.config.field_heigth)
            };
            if let Point::Fruit=self.field[rand_coords.y][rand_coords.x]{
                self.fruits.push(rand_coords)
            }
        }
        //nomnom
    }
    pub fn add_player(&mut self,nickname:String,player_soket:String){
        self.snekos.push(Sneko::new(nickname,player_soket))
    }
    pub fn remove_player(&self,nickname:String){
        
    }
}
#[derive(PartialEq,Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}
impl Direction{
    pub fn opposite(&self)->Direction{
        match self{
            Direction::Up=>{Direction::Down}
            Direction::Down=>{Direction::Up}
            Direction::Left=>{Direction::Right}
            Direction::Right=>{Direction::Left}
        }
    }
}
#[derive(Clone,Debug)]
pub struct Coords {
    pub x:usize,
    pub y:usize,
}

pub struct Sneko {
    nickname:String,
    player_soket: String,
    pub coords: Coords,
    pub direction: Direction,
    pub body_cells: Vec<Coords>,
    pub length: usize,
    pub alive: bool,
    pub prev_direction: Direction
}

impl Sneko {
    pub fn new(nickname:String,player_soket:String)-> Sneko{
        Sneko {
            nickname,
            player_soket,
            coords: Coords{x:0,y:0}, 
            direction: Direction::Right,
            body_cells: vec![],//yet to implement
            length: 3,
            alive: true,    
            prev_direction: Direction::Right
        }
    }
    pub fn slither(&mut self){
        match self.direction {
            Direction::Up => {self.coords.y+=1},
            Direction::Down =>{  
                match self.coords.y {
                    0 => {self.alive = false}
                    _ => {self.coords.y-=1}
                } },
            Direction::Left =>{ 
                match self.coords.x {
                    0 => {self.alive = false}
                    _ => {self.coords.x-=1}
                }}
            Direction::Right =>{self.coords.x+=1},
        }
        //wiggly wiggle
    }
}


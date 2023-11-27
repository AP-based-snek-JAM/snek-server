use rand::Rng;
//use std::rc::{ Weak,Rc };

pub struct Config {
    pub grid_width:usize,
    pub grid_heigth:usize,
    pub fruit_count:usize,
}
impl Config {
    pub fn new(grid_width:usize,grid_heigth:usize,fruit_count:usize) -> Config {
        Config{grid_width,grid_heigth,fruit_count}
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
    pub grid: Box<[Box<[Point]>]>,
    config: Config,
    snekos:Vec<Sneko>,
    fruits:Vec<Coords>
}
impl Game{
    pub fn new(config:Config) -> Game {
        Game {
            soket: String::new(),
            grid: vec![
                vec![Point::Empty;config.grid_width].into_boxed_slice();config.grid_heigth
            ].into_boxed_slice(), 
            config,
            snekos:vec![],
            fruits:vec![]
        }
    }
    pub fn print_grid(&self){
        self.grid.iter().for_each(|row|{ 
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
        self.update_grid_values();
    }
    pub fn spawn_snekos(&mut self){
        let number_of_snekos = self.snekos.len();
        self.snekos.iter_mut().enumerate().for_each(|(i,sneko)|{
            sneko.alive=true;
            let side_direction = match i%4{
                0 => {Direction::Left}
                1 => {Direction::Right}
                2 => {Direction::Up}
                3 => {Direction::Down}
                _ => unreachable!()
            };
            sneko.prev_direction = side_direction.opposite();
            sneko.direction = side_direction.opposite();
            let snekos_on_side = (number_of_snekos/4) + {
                match i%4 <= number_of_snekos%4 {
                    true => {1}
                    false => {0}
                }
            };
            let nth_on_side=1+(i/4);
            let spacing = self.config.grid_width as f32 / ((snekos_on_side+1) as f32);
            sneko.coords = {match side_direction {
                Direction::Up => {
                    Coords{x:(nth_on_side as f32 * spacing).ceil() as usize,y:self.config.grid_heigth-1}}
                Direction::Down => {
                    Coords{x:(nth_on_side as f32 * spacing).ceil() as usize,y:0}}
                Direction::Left => {
                    Coords{x:0,y:(nth_on_side as f32 * spacing).ceil() as usize}}
                Direction::Right => {
                    Coords{x:self.config.grid_width-1,y:(nth_on_side as f32 * spacing).ceil() as usize}}
            }};
            println!("{:?},snekos on side:{},nth_on_side:{},{},{:?}",side_direction,snekos_on_side,nth_on_side,spacing,sneko.coords)
        });
    }
        
    fn update_grid_values(&mut self){ 
        self.snekos.iter().for_each(|snek|{
            if let true = snek.alive{
                self.grid[snek.coords.y][snek.coords.x] = Point::Sneko(snek.length);    
            }
       })
    }

    pub fn change_player_direction(&mut self,player_id:usize,direction:Direction){
        //let reference_to_sneko: Option<&Sneko> = self.snekos.iter().find(|snek| snek.player_id == player_id);
        if let Some(sneko) = self.snekos.iter_mut().find(|snek| snek.player_id == player_id){
            if sneko.prev_direction.opposite() != direction{
                sneko.direction = direction;
            }
        }
    }
    pub fn tick(&mut self){
        if self.number_of_alive_snekos() == 0 {
            println!("GAME OVER");
            return
        }
        //temporary
        
        //temporary
        self.decrease_grid_values();
        self.slither_alive_snekos();
        self.handle_movement();
        print! ("\x1B[2J\x1B[1;1H");
        self.print_grid();
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
                    let point = &mut self.grid[sneko.coords.y][sneko.coords.x];
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
    fn decrease_grid_values(&mut self){
        self.grid.iter_mut().for_each(|row|{
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
            if snek.coords.x>=self.config.grid_width || snek.coords.y>=self.config.grid_heigth{    
                snek.alive = false;
            }
        })
    }
    fn spawn_fruits(&mut self){
        while self.fruits.len() < self.config.fruit_count{
            let mut rnd = rand::thread_rng();
            let rand_coords = Coords{
                x: rnd.gen_range(0..self.config.grid_width),
                y: rnd.gen_range(0..self.config.grid_heigth)
            };
            if let Point::Fruit=self.grid[rand_coords.y][rand_coords.x]{
                self.fruits.push(rand_coords)
            }
        }
        //nomnom
    }
    pub fn add_player(&mut self,nickname:String) -> usize{
        let mut id=0;
        while true == self.snekos.iter().any(|sneko|sneko.player_id==id){
            id+=1; 
        }
        self.snekos.push(Sneko::new(nickname,id));
        id
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
    player_id: usize,
    pub coords: Coords,
    pub direction: Direction,
    pub body_cells: Vec<Coords>,
    pub length: usize,
    pub alive: bool,
    pub prev_direction: Direction
}

impl Sneko {
    pub fn new(nickname:String,player_id:usize)-> Sneko{
        Sneko {
            nickname,
            player_id,
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


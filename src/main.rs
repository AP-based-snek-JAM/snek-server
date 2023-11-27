use clap::Parser;
use sneko_engine::Game;
use std::{ 
    net::{
        IpAddr, 
        Ipv4Addr, 
        SocketAddr,
        TcpListener,
        TcpStream,
    },
    time::Duration,
    thread::sleep,
    error::Error,
    io::{
        BufReader,
        BufRead,
        Write
    },
    sync::mpsc
};



mod sneko_engine;

fn main() {
//    let cli = Cli::parse();
//    if let Err(e) = run(cli) {
//        println!("Application error: {e}");
//        std::process::exit(1);
//    }
    start_tcp_server();
}

enum MsgFromClient {
    ShowActiveGames,
    JoinGame(u16),
    SendGameData(u16),
    ChangePlayerDirection(u16,u16),
}

//enum ThreadMessage {
//    ChangePlayerDirection(usize,sneko_engine::Direction),
//    AddPlayer()
//    //todo
//
//
//}

fn run(cli:Cli)-> Result<(),Box<dyn Error>>{
    let game_config=sneko_engine::Config::new(cli.width,cli.heigth,cli.fruit_count);
    let mut game = sneko_engine::Game::new(game_config);
    let mut players:Vec<usize> = vec![];
    players.push(game.add_player("Hejdula".to_string()));
    players.push(game.add_player("2".to_string()));
    players.push(game.add_player("3".to_string()));
    players.push(game.add_player("4".to_string()));
    players.push(game.add_player("5".to_string()));
    players.push(game.add_player("6".to_string()));
    game.start();
    game.change_player_direction(players[0], sneko_engine::Direction::Up);
    (1..12).for_each(|_|{game.tick();println!("");sleep(Duration::from_millis(400))});
    return Ok(());
}

fn start_tcp_server(){
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878);
    let listener = TcpListener::bind(socket).unwrap();
    println!("Starting TCP server on {socket}");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);

        println!("Connection handled");
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("got here");
    //let buf_reader = BufReader::new(&mut stream);
    let mut response = "OK\r\n\r\n";
    response = "heeeej";
    println!("and here");
    stream.write_all(response.as_bytes()).unwrap();
}

//fn spawn_game_thread(){
//    let (thread_tx, main_rx) = mpsc::channel();
//    let (main_tx, thread_rx) = mpsc::channel();
//    // Spawning a thread to send messages
//    let thread = std::thread::spawn(move || {
//        thread_tx.send("Hello from thread").unwrap();
//        let received = thread_rx.recv().unwrap();
//        println!("{}",received);
//        std::thread::sleep(Duration::from_millis(1000));
//    });
//
//    // Receiving the message
//    main_tx.send("Hello from main").unwrap();
//    let received = main_rx.recv().unwrap();
//    println!("Received: {}", received);
//    thread.join().expect("thread panicked");
//    println!("joined");
//    
//
//}


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

use std::io::{Read, Write as _, stdin};
use std::net::{TcpListener, TcpStream};

/// Cross goes first
enum Player {
    Cross,
    Circle,
}

// Tic tac toe
#[derive(Default)]
struct GameState {
    board: [Option<Player>; 9],
}
impl GameState {
    pub fn print_board(&self) {
        println!("-------");
        for r in 0..3 {
            print!("|");
            for c in 0..3 {
                match self.get(r, c) {
                    None => print!(" "),
                    Some(Player::Cross) => print!("x"),
                    Some(Player::Circle) => print!("o"),
                }
                print!("|");
            }
            println!("-------");
        }
    }

    pub fn winner(&self) -> Option<Player> {
        // rows and columns
        for i in 0..3 {
            if (0..3)
                .map(|j| self.get(i, j))
                .all(|x| matches!(x, Some(Player::Cross)))
            {
                return Some(Player::Cross);
            }
            if (0..3)
                .map(|j| self.get(i, j))
                .all(|x| matches!(x, Some(Player::Circle)))
            {
                return Some(Player::Circle);
            }
        }
        for i in 0..3 {
            if (0..3)
                .map(|j| self.get(j, i))
                .all(|x| matches!(x, Some(Player::Cross)))
            {
                return Some(Player::Cross);
            }
            if (0..3)
                .map(|j| self.get(j, i))
                .all(|x| matches!(x, Some(Player::Circle)))
            {
                return Some(Player::Circle);
            }
        }

        // Diagonals
        if (0..3)
            .map(|j| self.get(j, j))
            .all(|x| matches!(x, Some(Player::Cross)))
            || (0..3)
                .map(|j| self.get(j, 2 - j))
                .all(|x| matches!(x, Some(Player::Cross)))
        {
            return Some(Player::Cross);
        }
        if (0..3)
            .map(|j| self.get(j, j))
            .all(|x| matches!(x, Some(Player::Circle)))
            || (0..3)
                .map(|j| self.get(j, 2 - j))
                .all(|x| matches!(x, Some(Player::Circle)))
        {
            return Some(Player::Circle);
        }

        None
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Player> {
        assert!(row < 3);
        assert!(col < 3);
        self.board[row * 3 + col].as_ref()
    }
}

fn get_coordinate_input(ordinate_x: bool) -> u8 {
    let mut string_buf = String::new();
    loop {
        if ordinate_x {
            println!("X: ");
        } else {
            println!("Y: ");
        }

        stdin().read_line(&mut string_buf).unwrap();
        match string_buf.as_str().parse::<u8>() {
            Ok(x) => break x,
            _ => println!("Invalid input! Enter (0 - 2)!"),
        }
    }
}

fn server() {
    println!("Input address to listen on: ");
    let mut address = String::new();
    stdin()
        .read_line(&mut address)
        .expect("Unable to read address from input");
    let address = address.trim();

    let listener = TcpListener::bind(&address).unwrap_or_else(|err| {
        panic!("Unable to get tcp listener: {}", err);
    });
    println!("Listening on {}", address);

    loop {
        if let Ok((mut stream, a)) = listener.accept() {
            println!("Connection accepted from address: {}", a);

            println!("Starting new game");
            let mut game = GameState::default();

            let mut string_buf = String::new();

            // 0 represents server
            // 1 represents client
            let player_order: [u8; 2] = loop {
                println!("Choose O or X: ");
                stdin()
                    .read_line(&mut string_buf)
                    .expect("Unable to read player selection");
                string_buf.make_ascii_lowercase();

                match string_buf.as_str() {
                    "o" => break [1, 0],
                    "x" => break [0, 1],
                    _ => {
                        println!("Invalid player selected!")
                    }
                }
            };

            // Cross goes first
            let mut turn = 0;
            while game.winner().is_none() {
                game.print_board();
                if turn == 0 {
                    println!("Cross (X) to play,");
                } else {
                    println!("Circle (O) to play,");
                }

                // Server
                if player_order[turn] == 0 {
                    println!("What's your move?");
                    let x = get_coordinate_input(true);
                    let y = get_coordinate_input(false);
                } else {
                    println!("Waiting for other player...");

                    stream
                        .read_to_string(&mut string_buf)
                        .unwrap_or_else(|err| {
                            panic!("ERROR: Unable to read tcp stream into string: {}", err);
                        });
                    println!("Received string: {}", &string_buf);
                }
                turn = turn ^ 1;
            }

        }
    }
}

fn client() {
    println!("Input address to write to: ");
    let mut address = String::new();
    stdin()
        .read_line(&mut address)
        .expect("Unable to read address from input");
    let address = address.trim();
    println!("Set address to: {}", &address);

    loop {
        let mut string = String::new();
        println!("What to say?");
        stdin()
            .read_line(&mut string)
            .expect("Unable to read from stdin");

        let mut stream = TcpStream::connect(&address).unwrap_or_else(|err| {
            panic!("Unable to connect to {}: {}", address, err);
        });
        stream.write(string.as_bytes()).unwrap_or_else(|err| {
            panic!("Unable to write into tcp address: {}", err);
        });
        stream.flush().expect("Unable to flush stream");

        println!("Successfully written {} into {}", string, address);
    }
}

fn main() {
    println!("Select client or server: ");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap_or_else(|err| {
        panic!("Unable to read input into string: {}", err);
    });

    match input.as_str().trim() {
        "server" => server(),
        "client" => client(),
        x => {
            eprintln!("Invalid mode: {}", x);
            return;
        }
    }
}


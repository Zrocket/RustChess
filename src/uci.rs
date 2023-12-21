use std::io;
use std::str::SplitWhitespace;

const ENGINE_NAME: &'static str = "RustChess";
const AUTHOR_NAME: &'static str = "Drake Murphy";

/// UCI engine
/// name: engine name
/// author: engine author
pub struct Uci {
    name: String,
    author: String,
    debug: bool,
}

impl Default for Uci {
    fn default() -> Uci {
        Uci {
            name: ENGINE_NAME.to_string(),
            author: AUTHOR_NAME.to_string(),
            debug: false,
        }
    }
}

/// Xboard engine
pub struct Xboard;

impl Uci {
    fn debug(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                "on" => self.debug = true,
                "off" => self.debug = false,
                _ => todo!(),
            }
        }
    }

    fn is_ready(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                _ => todo!(),
            }
        }
    }

    fn set_option(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            if command == "name" {
                if let Some(command) = params.next() {
                    if command == "value" {}
                }
            }
        }
        println!("");
    }

    fn register(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                "later" => todo!(),
                "name" => todo!(),
                _ => todo!(),
            }
        }
    }

    fn position(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            if command == "fen" {
                if let Some(command) = params.next() {
                    decode_fen(command);
                }
            }
        }
    }

    fn go(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                _ => todo!(),
            }
        }
    }

    fn stop(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                _ => todo!(),
            }
        }
    }

    fn ponder_hit(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                _ => todo!(),
            }
        }
    }

    fn quit(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                _ => todo!(),
            }
        }
    }

    fn uci_new_game(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                _ => todo!(),
            }
        }
    }

    fn perft(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                _ => todo!(),
            }
        }
    }

    fn test(&self, params: &mut SplitWhitespace) {
        if let Some(command) = params.next() {
            match command {
                _ => todo!(),
            }
        }
    }

    fn engine_loop(&self) {
        println!("id name {}", self.name);
        println!("id author {}", self.author);
        println!("uciok");
        loop {
            for line in io::stdin().lines() {
                let mut params = line.unwrap().split_whitespace();

                if let Some(command) = params.next() {
                    match command {
                        "debug" => self.debug(&mut params),
                        "isready" => self.is_ready(&mut params),
                        "setoption" => self.set_option(&mut params),
                        "register" => self.register(&mut params),
                        "position" => self.position(&mut params),
                        "go" => self.go(&mut params),
                        "stop" => self.stop(&mut params),
                        "ponderhit" => self.ponder_hit(&mut params),
                        "quit" => self.quit(&mut params),
                        "ucinewgame" => self.uci_new_game(&mut params),
                        "perft" => self.perft(&mut params),
                        "test" => self.test(&mut params),
                        _ => println!("Unkown command: {}", command),
                    }
                }
            }
        }
    }
}

fn decode_fen(fen: &str) {
    let mut parts = fen.split('/');
    for part in parts {
        for i in part.chars() {
            match i {
                '1' => todo!(),
                '2' => todo!(),
                '3' => todo!(),
                '4' => todo!(),
                '5' => todo!(),
                '6' => todo!(),
                '7' => todo!(),
                '8' => todo!(),
                'P' => todo!(),
                'N' => todo!(),
                'B' => todo!(),
                'R' => todo!(),
                'Q' => todo!(),
                'K' => todo!(),
                'p' => todo!(),
                'n' => todo!(),
                'b' => todo!(),
                'r' => todo!(),
                'q' => todo!(),
                'k' => todo!(),
                _ => todo!(),
            }
        }
    }
}

/// Main entry point
pub fn entry() {
    for line in io::stdin().lines() {
        let mut params = line.unwrap().split_whitespace();

        if let Some(command) = params.next() {
            match command {
                "uci" => {
                    let engine = Uci::default();
                    engine.engine_loop();
                }
                _ => println!("Unkown command: {}", command),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}

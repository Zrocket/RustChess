use std::io;

const ENGINE_NAME :&'static str = "";

pub fn main_loop() {
    for line in io::stdin().lines() {
        let mut params = line.unwrap().split_whitespace();

        if let Some(command) = params.next() {
            match command {
                "uci" => todo!(),
                "isready" => todo!(),
                "setoption" => todo!(),
                "ucinewgame" => todo!(),
                "position" => todo!(),
                "stop" => todo!(),
                "quit" => return,
                "perft" => todo!(),
                "test" => todo!(),
                "go" => todo!(),
                _ => println!("Unkown command: {}", command)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
    }
}

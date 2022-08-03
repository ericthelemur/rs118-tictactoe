use std::{fmt, io};


#[derive(PartialEq, Eq, Copy, Clone)]
enum Player {
    X, O
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

struct Board {
    board: [[Option<Player>; 3]; 3]
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  A B C \n").unwrap();
        let mut i = 0;
        for row in self.board.iter() {
            i += 1;
            write!(f, "{} ", i).unwrap();
            for cell in row.iter() {
                match cell {
                    Some(p) => write!(f, "{} ", p).unwrap(),
                    None => write!(f, ". ").unwrap(),
                }
            }
            write!(f, "\n").unwrap()
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GuessError {
    ReadError,
    GuessLengthError,
    FirstSymbolError,
    SecondSymbolError
}

impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            GuessError::ReadError => "Error reading input",
            GuessError::GuessLengthError => "Guess in the format 'A1'.",
            GuessError::FirstSymbolError => "The first letter should be A, B or C.",
            GuessError::SecondSymbolError => "The second digit should be 1, 2 or 3.",
            _ => "Error"
        })
    }
}

fn get_guess() -> Result<(usize, usize), GuessError> {
    println!("Which square?");
    // Get input
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .or(Err(GuessError::ReadError))?;
    
    let mut chars = guess.chars();
    // First char
    let fst = chars.next().ok_or(GuessError::FirstSymbolError)?;
    let col: usize = match fst {
        'A' => 0, 'B' => 1, 'C' => 2,
        _ => { return Err(GuessError::FirstSymbolError) }
    };
    // Second char
    let snd = chars.next().ok_or(GuessError::SecondSymbolError)?;
    println!("{}", snd);
    let row: usize = match snd {
        '1' => 0, '2' => 1, '3' => 2,
        _ => { return Err(GuessError::SecondSymbolError) }
    };

    // if chars.next().is_some() { return Err(GuessError::GuessLengthError) }

    return Ok((row, col))
}


fn main() {
    let mut b = Board{board: [
        [Some(Player::X), None, None],
        [None, Some(Player::O), None],
        [None, None, None],
    ]};
    println!("\nBoard:\n{}", b);

    let mut current_player = Player::X;
    
    loop {
        println!("Current Player: {}", current_player);
        let (row, col) = match get_guess() {
            Err(e) => { println!("{}", e); continue; },
            Ok(o) => o,
        };
        println!("Guess is {}, {}", row, col);

        if b.board[row][col].is_none() {
            b.board[row][col] = Some(current_player);
            current_player = match current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            
            println!("\nBoard:\n{}", b);
        } else {
            println!("Space already occupied");
            continue;
        }
    }
}

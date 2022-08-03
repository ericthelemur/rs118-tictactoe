use std::fmt;


#[derive(PartialEq, Eq, Clone)]
enum Player {
    X, O
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
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
        for row in self.board.iter() {
            for cell in row.iter() {
                match cell {
                    Some(p) => write!(f, "{} ", p),
                    None => write!(f, ". "),
                }?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}


fn main() {
    let b = Board{board: [
        [None, Some(Player::O), None],
        [None, None, Some(Player::X)],
        [None, None, None],
    ]};

    println!("Test Board:\n{}", b);
}

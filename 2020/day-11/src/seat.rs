use std::fmt;

#[derive(Debug)]
pub enum Seat {
    Floor,
    Occupied,
    Empty,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Seat::Empty => write!(f, "L"),
            Seat::Floor => write!(f, "."),
            Seat::Occupied => write!(f, "#"),
        }
    }
}

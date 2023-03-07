#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition(rank, file)),
            _ => None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let ChessPosition(r1, f1) = self.0;
        let ChessPosition(r2, f2) = other.0;
        r1 == r2 || f1 == f2 || i32::abs(r1 - r2) == i32::abs(f1 - f2)
    }
}

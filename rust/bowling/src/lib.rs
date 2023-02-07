use self::{Error::*, NthThrow::*};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub enum NthThrow {
    First=1,
    Second=2,
    Third=3
}

pub struct BowlingGame {
    frames: u16,
    nth_throw: NthThrow,
    pins_left: u16,
    scores: Vec<u16>,
    is_complete: bool
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: 0,
            nth_throw: First,
            pins_left: 10,
            scores: Vec::new(),
            is_complete: false
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete {
            return Err(GameComplete);
        }
        if pins > self.pins_left {
            return Err(NotEnoughPinsLeft);
        }
        match self.nth_throw {
            First => {
                self.frames += 1;
                self.scores.push(pins);
                // strike
                if pins == 10 {
                    if self.frames == 10 {
                        self.nth_throw = Second;
                    } else {
                        self.scores.push(0);
                    }
                } else {
                    self.nth_throw = Second;
                    self.pins_left = 10 - pins;
                }
            }
            Second => {
                if self.frames == 10 {
                    if self.scores.last().unwrap() == &10 {
                        self.scores.push(pins);
                        self.nth_throw = Third;
                        if pins == 10 {
                            self.pins_left = 10;
                        } else {
                            self.pins_left = 10 - pins;
                        }
                    } else if self.pins_left == pins {
                        self.scores.push(pins);
                        self.nth_throw = Third;
                        self.pins_left = 10;
                    } else {
                        self.scores.push(pins);
                        self.scores.push(0);
                        self.scores.push(0);
                        self.scores.push(0);
                        self.is_complete = true;
                    }
                } else {
                    self.scores.push(pins);
                    self.nth_throw = First;
                    self.pins_left = 10;
                }
            }
            Third => {
                self.scores.push(pins);
                self.scores.push(0);
                self.scores.push(0);
                self.is_complete = true;
            }
        }
        return Ok(());
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_complete {
            let score = self.scores.windows(5).enumerate().map(|(i, arr)| {
                if i % 2 == 1 {
                    return 0;
                }
                if i == 18 || (arr[0] == 10 && arr[2] == 10) {
                    if i == 16 {
                        return arr.iter().take(4).sum();
                    }
                    return arr.iter().sum();
                } else if arr[0] == 10 {
                    return arr.iter().take(4).sum();
                } else if arr[0] + arr[1] == 10 {
                    return arr.iter().take(3).sum();
                } else {
                    return arr.iter().take(2).sum();
                }
            }).sum();
            Some(score)
        } else {
            None
        }
    }
}

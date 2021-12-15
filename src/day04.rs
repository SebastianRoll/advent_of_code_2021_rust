use std::fs::File;
use std::io::{self, BufRead};

const BINGO_SIZE: usize = 5;

type IsBingo = bool;
type Row = usize;
type Col = usize;

#[derive(Copy, Clone, Debug)]
struct BingoCell {
    value: u8,
    marked: bool,
}

impl BingoCell {
    fn new(value: u8) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

#[derive(Debug)]
pub struct BingoBoard {
    board: [BingoCell; BINGO_SIZE * BINGO_SIZE],
    ended: bool,
}

impl BingoBoard {
    pub fn new(buf: &str) -> Self {
        let mut board = [BingoCell::new(0); BINGO_SIZE * BINGO_SIZE];

        buf.split_whitespace().enumerate().for_each(|(i, numstr)| {
            let num: u8 = numstr.parse().unwrap();
            //let cell = BingoCell::new(num);
            board[i].value = num;
        });
        BingoBoard {
            board,
            ended: false,
        }
    }

    pub fn mark(&mut self, value: u8) -> Option<u32> {
        if self.ended {
            return None;
        }
        if let Some(idx) = self.board.iter_mut().position(|c| c.value == value) {
            let mut cell = &mut self.board[idx];
            cell.marked = true;
            if self.check_bingo(idx) {
                self.ended = true;
                return Some(self.score(value));
            }
        }
        None
    }

    fn idx_to_row_col(&self, idx: usize) -> (Row, Col) {
        (idx / BINGO_SIZE, idx % BINGO_SIZE)
    }

    fn row_col_to_idx(&self, row: Row, col: Col) -> usize {
        row * BINGO_SIZE + col
    }

    fn check_bingo(&self, idx: usize) -> IsBingo {
        let (row, col) = self.idx_to_row_col(idx);

        let rows_incr: Vec<usize> = (0..BINGO_SIZE).collect();
        let cols_incr: Vec<usize> = (0..BINGO_SIZE).collect();
        let rows_const = [row; BINGO_SIZE];
        let cols_const = [col; BINGO_SIZE];

        // check horizontal
        let marked_hor = self.check_range(&rows_incr, &cols_const);

        // check vertical
        let marked_ver = self.check_range(&rows_const, &cols_incr);

        // check diagonal
        let marked_diag_down = self.check_range(&rows_incr, &cols_incr);
        let rows_decr: Vec<usize> = rows_incr.into_iter().rev().collect();
        let marked_diag_up = self.check_range(&rows_decr, &cols_incr);

        marked_hor || marked_ver || marked_diag_down || marked_diag_up
    }

    fn check_range(&self, rows: &[usize], cols: &[usize]) -> IsBingo {
        rows.iter()
            .zip(cols.iter())
            .filter(|(&r, &c)| {
                let check_idx = self.row_col_to_idx(r, c);
                self.board[check_idx].marked
            })
            .count()
            == BINGO_SIZE
    }

    fn score(&self, winning_number: u8) -> u32 {
        self.board
            .iter()
            .filter(|&c| !c.marked)
            .map(|&c| c.value as u32)
            .sum::<u32>()
            * winning_number as u32
    }
}

pub struct BingoGame {
    boards: Vec<BingoBoard>,
    numbers: Vec<u8>,
}

impl BingoGame {
    pub fn read_file(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let mut lines = io::BufReader::new(file).lines();

        let numbers: Vec<u8> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|numstr| numstr.parse::<u8>().unwrap())
            .collect();

        lines.next();

        let mut boards = Vec::new();
        let mut buf = String::new();
        while let Some(Ok(l)) = lines.next() {
            if l != "" {
                buf.push_str(&l);
                buf.push_str(" ");
            } else {
                boards.push(BingoBoard::new(&buf));
                buf.clear();
            }
        }
        BingoGame {
            numbers,
            boards: boards,
        }
    }

    pub fn play_game(&mut self) -> Option<u32> {
        for number in &self.numbers {
            for board in &mut self.boards {
                if let Some(score) = board.mark(*number) {
                    return Some(score);
                }
            }
        }
        None
    }

    pub fn play_game_find_loser(&mut self) -> Option<u32> {
        let mut in_play = self.boards.len();
        for number in &self.numbers {
            for board in &mut self.boards {
                if let Some(score) = board.mark(*number) {
                    if in_play == 1 {
                        return Some(score);
                    } else {
                        in_play -= 1;
                    }
                }
            }
        }
        None
    }
}

pub fn part1(path: &str) -> u32 {
    let mut game = BingoGame::read_file(path);
    game.play_game().unwrap()
}

pub fn part2(path: &str) -> u32 {
    let mut game = BingoGame::read_file(path);
    game.play_game_find_loser().unwrap()
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_from_file() {
        let bingo = BingoGame::read_file("data/input04.txt");

        //assert_eq!(bingo.numbers, [10]);
        assert_eq!(bingo.boards.len(), 2);
    }
}

*/

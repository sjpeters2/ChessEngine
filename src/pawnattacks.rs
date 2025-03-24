use crate::utils::*;

pub struct PawnAttacks {
    white_forward_moves: Vec<Bitboard>,
    white_diagonal_moves: Vec<Bitboard>,
    black_forward_moves: Vec<Bitboard>,
    black_diagonal_moves: Vec<Bitboard>,
}
impl  PawnAttacks {
    fn init() -> Self {
        let mut white_forward = vec![];
        let mut white_diagonal = vec![];
        let mut black_forward = vec![];
        let mut black_diagonal = vec![];

        for row in 1..=8 {
            for col in 1..=8 {
                let f = forward_move(row, col,Color::White);
                let d = diagonal_move(row, col,Color::White);

                white_forward.push(f);
                white_diagonal.push(d);

                let f = forward_move(row, col,Color::Black);
                let d = diagonal_move(row, col,Color::Black);

                black_forward.push(f);
                black_diagonal.push(d);
            }
        }
        return Self {
            white_forward_moves: white_forward,
            white_diagonal_moves: white_diagonal,
            black_forward_moves: black_forward,
            black_diagonal_moves: black_diagonal,
        }
    } 
}

fn forward_move(row: i32, col:i32, color:Color) -> Bitboard {
    if row == 1 || row == 8 {
        return 0;
    }
    let mut bitboard = 0;
    if color == Color::White {
        if row < 8 {
            bitboard |= set_bit(row + 1, col);
        }
       if row == 2 {
            bitboard |= set_bit(row + 1, col);
            bitboard |= set_bit(row + 2, col);
        }
    } else {
        if row > 1 {
            bitboard |= set_bit(row - 1, col);
        }
        if row == 7 {
            bitboard |= set_bit(row - 2, col);
        }
    }
    bitboard
}

fn diagonal_move(row: i32, col:i32, color:Color) -> Bitboard {
    if row == 1 || row == 8 {
        return 0;
    } 

    let mut bitboard = 0;
    if color == Color::White {
        if row < 8 {
            bitboard |= set_bit(row + 1, col + 1);
            bitboard |= set_bit(row + 1, col - 1);
        }
    } else {
        if row > 1 {
            bitboard |= set_bit(row - 1, col + 1);
            bitboard |= set_bit(row - 1, col - 1);
        }
    }
    bitboard
}


#[cfg(test)]

mod tests {
    use super::*;

    // #[test]
    // fn test_pawn_forwardmoves(){
    //     let pawns = PawnAttacks::init();
    //     let row = 4;
    //     let col = 4;
    //     let idx = (row - 1) * 8 + col - 1;
    //
    //     println!("Here is the forward White Pawn moves: \n{}", bitboard_to_string(pawns.white_forward_moves[idx], Some(idx)));
    //     println!("Here is the diagonal White Pawn moves: \n{}", bitboard_to_string(pawns.white_diagonal_moves[idx], Some(idx)));
    //     println!("Here is the forward Black Pawn moves: \n{}", bitboard_to_string(pawns.black_forward_moves[idx], Some(idx)));
    //     println!("Here is the diagonal Black Pawn moves: \n{}", bitboard_to_string(pawns.black_diagonal_moves[idx], Some(idx)));
    // }

    #[test]
    fn test_pawn_moves_second_row(){
        let pawns = PawnAttacks::init();
        let row = 4;
        let col = 8;
        let idx = (row - 1) * 8 + col - 1;
        println!("Here is the forward White Pawn moves: \n{}", bitboard_to_string(pawns.white_forward_moves[idx], Some(idx)));
        println!("Here is the diagonal White Pawn moves: \n{}", bitboard_to_string(pawns.white_diagonal_moves[idx], Some(idx)));
        println!("Here is the forward Black Pawn moves: \n{}", bitboard_to_string(pawns.black_forward_moves[idx], Some(idx)));
        println!("Here is the diagonal Black Pawn moves: \n{}", bitboard_to_string(pawns.black_diagonal_moves[idx], Some(idx)));
    }

    #[test]
    fn test_middle_row_white_pawn(){
        for row in 3..=7 {
            for col in 1..=8 {
                let bitboard = forward_move(row, col, Color::White);
                let lsb = bit_scan(bitboard);

                let expected_lsb = (col - 1) + (row + 1 - 1) * 8;

                assert_eq!(lsb, expected_lsb as usize);
            }
        }
    }

}


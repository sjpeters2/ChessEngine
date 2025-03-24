use crate::utils::print_bitboard;
use crate::utils::set_bit;


const EMPTYBOARD:u64 = u64::MIN;
const FULLBOARD:u64 = u64::MAX;


pub struct Bitboard {
    board: u64
}

impl Bitboard {

    /// Initialize a bitboard to an empty set
    fn init() -> Self {
        Bitboard {board:EMPTYBOARD}
    }

    /// Sets the bit at row,col to 1
    fn set_bit(&mut self, row: i32, col: i32){
        self.board = set_bit(row, col);
    } 

    fn print_bitboard (self) {
        print_bitboard(self.board, None); 
    }

    fn get_board_as_u64(self) -> u64 {
        self.board
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_Bitboard_init(){
        let emptyboard = Bitboard::init();
     }

    #[test]
    fn test_print_empty_board(){
        let emptyboard = Bitboard::init();
        emptyboard.print_bitboard();
    }

    #[test]
    fn test_getboard_as_u64(){
        let emptyboard = Bitboard::init();
        let u64num = emptyboard.get_board_as_u64();
        println!("The u64num of the empty bitboard is: {}",u64num);
    }

    #[test]
    fn test_set_bit(){
        let mut emptyboard = Bitboard::init();
        emptyboard.set_bit(2,2);
        emptyboard.print_bitboard();
    }

    #[test]
    fn test_getboard_with_bit_set(){
        let mut emptyboard = Bitboard::init();
        emptyboard.set_bit(2,2);
        let u64num = emptyboard.get_board_as_u64();
        println!("The u64num of the set bitboard is: {}",u64num);
    }




}

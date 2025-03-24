// Magic Table to enable fast implementation of a bit scan function
pub type Bitboard = u64;


#[derive(Debug,PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black
}

static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23,
    3, 12, 16, 59, 41, 19, 24, 54,
    4, 64, 13, 10, 17, 62, 60, 28,
    42, 30, 20, 51, 25, 44, 55, 47,
    5, 32, 64, 38, 14, 22, 11, 58,
    18, 53, 63, 9, 61, 27, 29, 50,
    43, 46, 31, 37, 21, 57, 52, 8,
    26, 49, 45, 36, 56, 7, 48, 35,
    6, 34, 33];

// turns the first non-zero bit in a u64
pub fn bit_scan(bit: u64) -> usize {
    let one_bit = (bit ^ (bit - 1)) ^ (!bit & (bit - 1));
    let remainder = (one_bit % 67) as usize;
    return MOD67TABLE[remainder];
} 

pub fn bit_scan_backwards(bit: u64) -> usize {
    // 0010000 = 16 = 2^4 index of 1 = ln( 0010000 ) = 4
    // 0010010 = 18 = 2^4.2.. -> ln(0010010) = 4.2
    (bit as f64).log2().floor() as usize
}

pub fn row_col_to_index(row:i32, col:i32) -> i32 {
    return (col - 1) + (row - 1 )*8;
}


pub fn set_bit (row: i32, col: i32) -> Bitboard {
    if row < 1 || row > 8 || col < 1 || col > 8 {
        return 0;
    } else {
        return 1 << ((col - 1) + (row - 1) * 8);
    }
}

pub fn print_bitboard (bitboard: Bitboard, mark:Option<usize>) {
    println!("{}", bitboard_to_string(bitboard, mark));
}

pub fn bitboard_to_string (bitboard: Bitboard, mark:Option<usize>) -> String {
    let mut row = "".to_owned();
    let mut board  = "".to_owned();

    for i in 0..64 {
        let value = (bitboard >> i) & 1;
        let s = if value == 0 {
            ".".to_owned()
        } else {
            value.to_string()
        };

        match mark {
            Some(idx) => {
                if i == idx {
                    row.push_str("X");
                }else {
                    row.push_str(&s);
                }
            }
            None => row.push_str(&s),
        }

        if (i + 1) % 8 == 0 {
            row.push_str("\n");
            board.insert_str(0, &row);
            row.clear();
        }
    }
    return board
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bit_scan_backwards(){
        for highest_bit in 0..64 {
            let mut bit = 1 << highest_bit;
            for other_bit in 0..highest_bit {
                if (other_bit + 37) % 3 != 0 {
                    bit |= 1 << other_bit;
                }
            }
            let bit_scan_result = bit_scan_backwards(bit);
            assert_eq!(highest_bit, bit_scan_result);
        }
    }

    #[test]
    fn bit_scan_works(){
        for i in 0..64 {
            let bit = (1 as u64) << i;
            let index = bit_scan(bit);
            assert_eq!(i, index);
        }
    }

    #[test]
    fn bit_scan_with_multiple_bits(){
        for lowest_bit in 0..64 {
            let mut bit = 1 << lowest_bit;

            for other_bit in (lowest_bit + 1)..64 {
                if (other_bit + 37) % 3 != 0 {
                    bit |= 1 << other_bit;
                }
            }

            let bit_scan_result = bit_scan(bit);
            assert_eq!(lowest_bit, bit_scan_result);
        }
    }

    #[test]
    fn bit_scan_works_highest_bit_is_1(){
        for i in 0..64 {
            let mut bit = (1 as u64) << i;
            bit |= (1 as u64) << 63;
            let index = bit_scan(bit);
            assert_eq!(i, index);
        }
    }
}

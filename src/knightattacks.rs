
use crate::utils::*;

type Bitboard = u64;

pub struct KnightAttacks(Vec<Bitboard>);

impl KnightAttacks {
    fn init() -> Self {
        let mut attacks = vec![];
        for row in 1..=8 {
            for col in 1..=8{
                let attacks_from_this_square = knight_attacks(row, col);
                attacks.push(attacks_from_this_square);
            }
        }
        Self(attacks)
    }
}

fn knight_attacks(row: i32, col: i32) -> Bitboard {
    let attack_pairs = [(1,2), (1,-2), (-1,2), (-1,-2), (2,1), (2,-1), (-2, 1), (-2, -1)];
    let mut bitboard:Bitboard = 0;
    for (r,c) in attack_pairs {
        bitboard |= set_bit(row + r, col + c);
    }
    return bitboard;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_attacks_can_initialize() {
       let knight_attacks = KnightAttacks::init(); 
    }

    #[test]
    fn print_knight_attacks() {
       let knight_attacks = KnightAttacks::init(); 
       print_bitboard(knight_attacks.0[0], Some(0));
       print_bitboard(knight_attacks.0[40], Some(40));
       print_bitboard(knight_attacks.0[17], Some(17));
       print_bitboard(knight_attacks.0[55], Some(55));
    }
}

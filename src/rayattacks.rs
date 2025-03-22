use std::{i64, u64};

type Bitboard = u64;


static COL_MAP : [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

// Converts the index of the 64 to a human readable chess notation
// Will mainly be used for printing and debugging
pub fn index_to_coordinate(index: usize) -> (usize, usize) {
    let column = index % 8 + 1;
    let row = index / 8 + 1;

    return (column, row);
}

pub struct Rays {
    n_rays: Vec<u64>,
    e_rays: Vec<u64>,
    w_rays: Vec<u64>,
    s_rays: Vec<u64>,
    nw_rays: Vec<u64>,
    ne_rays: Vec<u64>,
    sw_rays: Vec<u64>,
    se_rays: Vec<u64>,
}

impl Rays {
    fn init() -> Self {
        let mut n_rays = vec![];
        let mut e_rays = vec![];
        let mut w_rays = vec![];
        let mut s_rays = vec![];
        let mut nw_rays = vec![];
        let mut ne_rays = vec![];
        let mut sw_rays = vec![];
        let mut se_rays = vec![];

       for i in 0..63 {
           let coord = index_to_coordinate(i);
           let col = coord.0;
           let row = coord.1;
            n_rays.push(make_ray(row as i64, col as i64, Direction::North));
            e_rays.push(make_ray(row as i64, col as i64, Direction::East));
            w_rays.push(make_ray(row as i64, col as i64, Direction::West));
            s_rays.push(make_ray(row as i64, col as i64, Direction::South));

            nw_rays.push(make_ray(row as i64, col as i64, Direction::Northwest));
            ne_rays.push(make_ray(row as i64, col as i64, Direction::Northeast));
            sw_rays.push(make_ray(row as i64, col as i64, Direction::Southwest));
            se_rays.push(make_ray(row as i64, col as i64, Direction::Southeast));
       } 

        
        return Self{
            n_rays:n_rays, 
            e_rays:e_rays,
            w_rays:w_rays,
            s_rays:s_rays,
            nw_rays:nw_rays,
            ne_rays:ne_rays,
            sw_rays:sw_rays,
            se_rays:se_rays,
        }
    }
}

fn n_ray(row: i64, col: i64) -> Bitboard {
    let mut bitboard = 0;

    for offset in 1..=8{
        if row + offset  > 8 {
            break;
        }
        bitboard = set_bit(bitboard, row + offset, col);
    }
    return bitboard;
}

enum Direction {
   North,
   South,
   East,
   West,
   Northwest,
   Northeast,
   Southwest,
   Southeast,
}


fn make_ray(row: i64, col: i64, direction: Direction) -> Bitboard {
    let mut bitboard: Bitboard = 0;

    for offset in 1..=8 {
        let (r_offset, c_offset) = match direction {
            Direction::North => (row + offset, col),
            Direction::South => (row - offset, col),
            Direction::East => (row, col + offset),
            Direction::West => (row, col - offset),
            Direction::Northwest => (row + offset, col - offset),
            Direction::Northeast => (row + offset, col + offset),
            Direction::Southwest => (row - offset, col - offset),
            Direction::Southeast => (row - offset, col + offset),
        };

        if r_offset < 1 || r_offset > 8 || c_offset < 0 || c_offset > 8{
            break;
        }

        //bitboard |= 1 << idx;
        bitboard = set_bit(bitboard, r_offset, c_offset);
    }
    bitboard
}

fn set_bit(bitboard: Bitboard, row: i64, col: i64) -> Bitboard{
    if row < 1 || row > 8 || col < 1 || col > 8 {
        return bitboard;
    }
    return bitboard | (1 << ((col - 1) + (row - 1) * 8))
}

fn bitboard_to_string (bitboard: Bitboard, mark:Option<usize>) -> String {
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
    fn test_index_to_position(){
        println!("Heres the result of index_to_Postion with an index of 0: {:?} ", index_to_coordinate(0));
    }

    #[test]
    fn make_general_ray(){
        let row = 4;
        let col = 5;

        let idx = col + ((row-1) * 8) - 1;
        println!("Heres the North  bitboard: -----------------------------\n{}\n------------------------------", bitboard_to_string(make_ray(row as i64,col as i64,Direction::North), Some(idx)));
        println!("Heres the South  bitboard: -----------------------------\n{}\n------------------------------", bitboard_to_string(make_ray(row as i64,col as i64,Direction::South), Some(idx)));
        println!("Heres the East  bitboard: -----------------------------\n{}\n------------------------------", bitboard_to_string(make_ray(row as i64,col as i64,Direction::East), Some(idx)));
        println!("Heres the West  bitboard: -----------------------------\n{}\n------------------------------", bitboard_to_string(make_ray(row as i64,col as i64,Direction::West), Some(idx)));
        println!("Heres the NorthEast  bitboard: -----------------------------\n{}\n------------------------------", bitboard_to_string(make_ray(row as i64,col as i64,Direction::Northeast), Some(idx)));
        println!("Heres the NorthWest  bitboard: -----------------------------\n{}\n------------------------------", bitboard_to_string(make_ray(row as i64,col as i64,Direction::Northwest), Some(idx)));
        println!("Heres the SouthWest  bitboard: -----------------------------\n{}\n------------------------------", bitboard_to_string(make_ray(row as i64,col as i64,Direction::Southwest), Some(idx)));
        println!("Heres the SouthEast  bitboard: -----------------------------\n{}\n------------------------------", bitboard_to_string(make_ray(row as i64,col as i64,Direction::Southeast), Some(idx)));

    }
}

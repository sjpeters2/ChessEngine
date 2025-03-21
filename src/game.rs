use std::u64;
use bitflags::bitflags;
use crate::utils::*;

type PiecePosition = u64;
static COL_MAP : [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

// Converts the index of the 64 to a human readable chess notation
// Will mainly be used for printing and debugging
pub fn index_to_position (index: usize) -> String {
    let column = index % 8;
    let row = index / 8 + 1;

    return format!("{}{}",COL_MAP[column], row);
}

pub fn  bit_to_position(bit: PiecePosition) -> Result<String, String> {
    if bit == 0 {
        return Err("No piece present!".to_string());
    } else {
        let onebit_index = bit_scan(bit);
        return Ok(index_to_position(onebit_index));
    }
}

pub fn position_to_bit(position: &str) -> Result<PiecePosition, String> {
    if position.len() != 2 {
        return Err(format!("Invalid length: {}, string: {}", position.len(), position));
    }

    let bytes = position.as_bytes();
    let byte0 = bytes[0];
    if byte0 < 97 || byte0 >= 97+8 {
        return Err(format!("Invalid Column Character: {}, string: {}", byte0 as char, position));
    }

    let column = (byte0 - 97) as u32;
    let byte1 = bytes[1];
    let row;
    match (byte1 as char).to_digit(10){
        Some(number) => if number < 1 || number > 8 {
            return Err(format!("Invalid row character: {}, string: {}", byte1 as char, position));
        } else {
            row = number - 1;
        },
        None => return Err(format!("Invalid row character: {}, string: {}", byte1 as char, position)),
    }
    let square_number = row * 8+ column;
    let bit = (1 as u64) << square_number;
    return Ok(bit)
}


#[derive(Debug,PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black
}

#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Debug, PartialEq)]
pub struct Piece {
    pub position:PiecePosition,
    pub color:Color,
    pub piece_type: PieceType,
}
impl Piece {
    fn to_string(&self) -> String{
        let mut result = match self.piece_type {
           PieceType::Pawn => "p ",
           PieceType::Knight => "n ",
           PieceType::Bishop => "b ",
           PieceType::Rook => "r ",
           PieceType::Queen => "q ",
           PieceType::King => "k "
        }.to_string();

        if self.color == Color::White {
            result.make_ascii_uppercase();
        }

        return result
    }


}

#[derive(Debug)]
enum Square {
    Empty,
    Occupied(usize),

}

bitflags! {
    /// Represents the castling states as for FEN Notation
    #[derive(Debug)]
    pub struct CastlingRights: u8 {
        const NONE           = 0;
        const WHITEKINGSIDE  = 1 << 0; //0b0001
        const WHITEQUEENSIDE = 1 << 1; //0b0010
        const BLACKKINGSIDE  = 1 << 2; //0b0100
        const BLACKQUEENSIDE = 1 << 3; //0b1000
        const ALL = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3); // 0b1111
    }
}




//Game Type. Will own the data. The controller
pub struct Game {
    pub pieces: Vec<Piece>,
    pub squares: Vec<Square>,
    pub active_color: Color,
    pub castling_rights: CastlingRights,
    pub en_passant:Option<PiecePosition>,
    pub ply: usize,
    pub fullmoves:usize,
}

 impl Game {

    fn push_piece_and_square(&mut self, position: usize, color: Color, piece_type: PieceType, index: &mut usize){
        self.pieces.push(Piece {position: (1 as u64) << position, color:color, piece_type:piece_type});
        self.squares.push(Square::Occupied(*index));
        *index += 1;
    } 

    fn push_empty_square(&mut self){
        self.squares.push(Square::Empty);
    }

    pub fn board_rep(&self) -> String {
        let mut board = "".to_owned();
        let mut temp = "".to_owned();

        for (i, square) in self.squares.iter().enumerate(){
            match square{
                Square::Empty => temp.push_str(&index_to_position(i)),
                Square::Occupied(idx) => temp.push_str(&self.pieces[*idx].to_string()),
            }
            if (i + 1) % 8 == 0 {
                temp.push_str("\n");
                board.insert_str(0, &temp );
                temp.clear();
            }
        }

        return board;
    }

    pub fn read_fen(fen: &str) -> Game {
        let mut game = Game {
            pieces:vec![],
            squares:vec![],
            active_color :Color::White,
            castling_rights:CastlingRights::ALL,
            en_passant:None,
            ply:0,
            fullmoves:1
        };

        if let Some((boardstr, rest)) = fen.split_once(' '){

            // If FEN Valid parse the first part for the main board
            let mut rows: Vec<&str> = boardstr.splitn(8, '/').collect();
            rows.reverse();

            let mut piece_index = 0;
            let mut piece_position = 0;

            for row in rows {
                let (pieces, squares) = parse_row(&row, piece_index,piece_position);

                for p in pieces {
                    game.pieces.push(p);
                    piece_index += 1;
                }

                for s in squares{
                    game.squares.push(s);
                }
            }

            let split_fen: Vec<&str> = rest.split(' ').collect();
            let color_to_move = split_fen[0];
            let castling_rights = split_fen[1];
            let en_passant = split_fen[2];
            let ply = split_fen[3];
            let fullmoves = split_fen[4];

            game.active_color = match color_to_move{
                "w" => Color::White,
                "b" => Color::Black,
                _ => panic!("Unknown color: {}", color_to_move)
            };
            
            let mut castling = CastlingRights::NONE;
            for ch in castling_rights.chars(){
                match ch {
                    'K' => castling |= CastlingRights::WHITEKINGSIDE,
                    'Q' => castling |= CastlingRights::WHITEQUEENSIDE,
                    'k' => castling |= CastlingRights::BLACKKINGSIDE,
                    'q' => castling |= CastlingRights::BLACKQUEENSIDE,
                    '-' => (),
                    other => panic!("Invalid Char: {}",other),
                }
            }
            game.castling_rights = castling;

            match en_passant {
                "-" => game.en_passant = None,
                s => match position_to_bit(s){
                    Err(msg) => panic!("{}",msg),
                    Ok(bit) => game.en_passant = Some(bit),
                }
            }

            match ply.parse(){
                Ok(number) => game.ply = number,
                Err(_) => panic!("Invalid ply: {}", ply),
            }

            match fullmoves.parse(){
                Ok(number) => game.fullmoves = number,
                Err(_) => panic!("Invalid fullmoves: {}", fullmoves),
            }
        } else {
            println!("No space found in fen");
        }
        return game
    }
}

fn parse_row(row: &str, mut piece_index: usize, mut piece_position: usize) -> (Vec<Piece>, Vec<Square>) {
    let mut pieces = Vec::new();
    let mut squares = Vec::new();
    let mut color;

    macro_rules! add_piece {
        ($piece_type:ident) => {
            {
                let piece = Piece {color: color, position: (1 as u64) << piece_position, piece_type: PieceType::$piece_type};
                let square = Square::Occupied(piece_index);
                pieces.push(piece);
                squares.push(square);
                piece_position += 1;
                piece_index += 1;
            }
            
        };
    }

    for ch in row.chars() {
        let is_upper =  ch.is_ascii_uppercase();
        color = if is_upper {Color::White} else {Color::Black};
        match ch.to_ascii_lowercase(){
            'r' => add_piece!(Rook),
            'n' => add_piece!(Knight),
            'b' => add_piece!(Bishop),
            'q' => add_piece!(Queen),
            'p' => add_piece!(Pawn),
            'k' => add_piece!(King),
            num => {
                match num.to_digit(10){
                    None => panic!("Invalid Input: {}", num),
                    Some(number) =>for _i in 0..number {
                        squares.push(Square::Empty);
                        piece_position += 1;
                    }
                }
            }
            
        }
    }
        return (pieces, Vec::from(squares));
}

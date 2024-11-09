// Board squares
enum Squares{
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1
}
// Sides to move
enum Sides{
    White,
    Black
}
// Bitboard macros

// returns != 0 if bit is set, 0 otherwise
macro_rules! get_bit {
    ($bitboard : ident, $square : ident) => {
        $bitboard & (1u64 << $square)
    };
}

// Set bit in square to 1
macro_rules! set_bit {
    ($bitboard : ident, $square : ident) => {
        $bitboard |= 1u64 << $square
    };
}

// Set bit in square to 0
macro_rules! pop_bit {
    ($bitboard : ident, $square : ident) => {
        if get_bit!($bitboard, $square) != 0 {$bitboard ^= (1u64 << $square)}
    };
}

// Bitboard representation
// TODO Move bitboard code to separate file
fn print_bitboard(bitboard: u64) {
    // loop over ranks
    for rank in 0..8 {
        // loop over files
        for file in 0..8 {
            let square: u8 = rank * 8 + file;

            // print rank index
            if file == 0 {
                print!("  {}  ", 8 - rank);
            }
            // print bit state (either 1 or 0)
            print!("{} ",  if get_bit!(bitboard, square) != 0 {1} else {0});
        }
        print!("\n");
    }
    // print board files (a-h)
    print!("\n     a b c d e f g h\n\n");

    print!("      Bitboard: {}\n\n", bitboard);
}
// Attack tables
//
//
// Pawn attack table
const NOT_A_FILE: u64 = 18374403900871474942;
const NOT_H_FILE: u64 = 9187201950435737471;
const NOT_HG_FILES: u64 = 4557430888798830399;
const NOT_AB_FILES: u64 = 18229723555195321596;

fn mask_pawn_attacks(square: u8, side: u8) -> u64 {

    // piece bitboard
    let mut bitboard: u64 = 0u64;

    // attack Bitboard
    let mut attacks: u64 = 0u64;

    set_bit!(bitboard, square);

    if side == 0 { //White pawn
        if bitboard >> 7 & NOT_A_FILE != 0 {attacks |= bitboard >> 7;}
        if bitboard >> 9 & NOT_H_FILE != 0 {attacks |= bitboard >> 9;}

    } else { //Black pawn
        if bitboard << 7 & NOT_H_FILE != 0 {attacks |= bitboard << 7;}
        if bitboard << 9 & NOT_A_FILE != 0 {attacks |= bitboard << 9;}
    }

    attacks
}

// init leaper pieces attacks
//
//

fn init_leaper_attacks() -> [[u64 ; 64]; 2]{

    let mut pawn_attacks: [[u64 ; 64]; 2] = [[0u64; 64]; 2];

    for side in 0..2 {
        for square in 0..64 {
            pawn_attacks[side][square] = mask_pawn_attacks(square as u8, side as u8);
        }
    }

    pawn_attacks
}


// Main function
//
//

fn main() {
    println!("\n\n Welcome to Rusty Knight \n\n");

    let pawn_attacks = init_leaper_attacks();

    let side = Sides::White as usize;

    for square in 0..64 {
        print_bitboard(pawn_attacks[side][square]);
    }
}

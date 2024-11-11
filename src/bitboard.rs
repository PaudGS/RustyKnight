// Board squares
pub enum Squares{
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
pub enum Sides{
    White,
    Black
}
// Bitboard macros

// returns != 0 if bit is set, 0 otherwise
#[macro_export]
macro_rules! get_bit {
    ($bitboard : ident, $square : ident) => {
        $bitboard & (1u64 << $square)
    };
}

// Set bit in square to 1
#[macro_export]
macro_rules! set_bit {
    ($bitboard : ident, $square : ident) => {
        $bitboard |= 1u64 << $square
    };
}


// Set bit in square to 0
#[macro_export]
macro_rules! pop_bit {
    ($bitboard : ident, $square : ident) => {
        if get_bit!($bitboard, $square) != 0 {$bitboard ^= (1u64 << $square)}
    };
}

// Bitboard representation
pub fn print_bitboard(bitboard: u64) {
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

//Useful bitboard constants
const NOT_A_FILE: u64 = 18374403900871474942;
const NOT_H_FILE: u64 = 9187201950435737471;
const NOT_HG_FILES: u64 = 4557430888798830399;
const NOT_AB_FILES: u64 = 18229723555195321596;

// Attack tables
//
//

// Pawn attack table
pub fn mask_pawn_attacks(square: u8, side: u8) -> u64 {

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

// Knight attack table

pub fn mask_knight_attacks(square: u8) -> u64 {

    // piece bitboard
    let mut bitboard: u64 = 0u64;

    // attack bitboard
    let mut attacks: u64 = 0u64;

    set_bit!(bitboard, square);

    if bitboard >> 17 & NOT_H_FILE != 0 {attacks |= bitboard >> 17;}
    if bitboard >> 15 & NOT_A_FILE != 0 {attacks |= bitboard >> 15;}
    if bitboard >> 10 & NOT_HG_FILES != 0 {attacks |= bitboard >> 10;}
    if bitboard >> 6 & NOT_AB_FILES != 0 {attacks |= bitboard >> 6;}

    if bitboard << 17 & NOT_A_FILE != 0 {attacks |= bitboard << 17;}
    if bitboard << 15 & NOT_H_FILE != 0 {attacks |= bitboard << 15;}
    if bitboard << 10 & NOT_AB_FILES != 0 {attacks |= bitboard << 10;}
    if bitboard << 6 & NOT_HG_FILES != 0 {attacks |= bitboard << 6;}

    attacks
}

// King attack table

pub fn mask_king_attacks(square: u8) -> u64 {

    // piece bitboard
    let mut bitboard: u64 = 0u64;

    // attack bitboard
    let mut attacks: u64 = 0u64;

    set_bit!(bitboard, square);

    if bitboard >> 8 != 0 {attacks |= bitboard >> 8;}
    if bitboard >> 9 & NOT_H_FILE != 0 {attacks |= bitboard >> 9;}
    if bitboard >> 7 & NOT_A_FILE != 0 {attacks |= bitboard >> 7;}
    if bitboard >> 1 & NOT_H_FILE != 0 {attacks |= bitboard >> 1;}

    if bitboard << 8 != 0 {attacks |= bitboard << 8;}
    if bitboard << 9 & NOT_A_FILE != 0 {attacks |= bitboard << 9;}
    if bitboard << 7 & NOT_H_FILE != 0 {attacks |= bitboard << 7;}
    if bitboard << 1 & NOT_A_FILE != 0 {attacks |= bitboard << 1;}

    attacks
}

// Bishop attack table

pub fn mask_bishop_attacks(square: u8) -> u64 {

    let mut attacks: u64 = 0u64;

    let tr: i8 = square as i8 / 8;
    let tf: i8 = square as i8 % 8;

    for (rank, file) in (tr + 1..=6).zip(tf + 1..=6) {attacks |= 1u64 << (rank * 8 + file);}
    for (rank, file) in (1..=tr - 1).rev().zip(tf + 1..=6) {attacks |= 1u64 << (rank * 8 + file);}
    for (rank, file) in (tr + 1..=6).zip((1..=tf - 1).rev()) {attacks |= 1u64 << (rank * 8 + file);}
    for (rank, file) in (1..=tr - 1).rev().zip((1..=tf - 1).rev()) {attacks |= 1u64 << (rank * 8 + file);}

    attacks
}

//Bishop attack table considering possible blocks

pub fn bishop_attacks_on_the_fly(square: u8, block: u64) -> u64 {

    let mut attacks: u64 = 0u64;

    let tr: i8 = square as i8 / 8;
    let tf: i8 = square as i8 % 8;

    for (rank, file) in (tr + 1..=7).zip(tf + 1..=7) {
        attacks |= 1u64 << (rank * 8 + file);
        if 1u64 << (rank * 8 + file) & block != 0 {break;}
    }
    for (rank, file) in (0..=tr - 1).rev().zip(tf + 1..=7) {
        attacks |= 1u64 << (rank * 8 + file);
        if 1u64 << (rank * 8 + file) & block != 0 {break;}
    }
    for (rank, file) in (tr + 1..=7).zip((0..=tf - 1).rev()) {
        attacks |= 1u64 << (rank * 8 + file);
        if 1u64 << (rank * 8 + file) & block != 0 {break;}
    }
    for (rank, file) in (0..=tr - 1).rev().zip((0..=tf - 1).rev()) {
        attacks |= 1u64 << (rank * 8 + file);
        if 1u64 << (rank * 8 + file) & block != 0 {break;}
    }

    attacks
}

// Rook attack table

pub fn mask_rook_attacks(square: u8) -> u64 {

    let mut attacks: u64 = 0u64;

    let tr: i8 = square as i8 / 8;
    let tf: i8 = square as i8 % 8;

    for rank in tr + 1..=6 {attacks |= 1u64 << (rank * 8 + tf);}
    for rank in (1..=tr - 1).rev() {attacks |= 1u64 << (rank * 8 + tf);}
    for file in tf +1..=6 {attacks |= 1u64 << (tr * 8 + file);}
    for file in (1..=tf - 1).rev() {attacks |= 1u64 << (tr * 8 + file);}

    attacks
}

//Rook attack table considering possible blocks

pub fn rook_attacks_on_the_fly(square: u8, block: u64) -> u64 {

    let mut attacks: u64 = 0u64;

    let tr: i8 = square as i8 / 8;
    let tf: i8 = square as i8 % 8;

    for rank in tr + 1..=7 {
        attacks |= 1u64 << (rank * 8 + tf);
        if 1u64 << (rank * 8 + tf) & block != 0 {break;}
    }
    for rank in (0..=tr - 1).rev() {
        attacks |= 1u64 << (rank * 8 + tf);
        if 1u64 << (rank * 8 + tf) & block != 0 {break;}
    }
    for file in tf +1..=7 {
        attacks |= 1u64 << (tr * 8 + file);
        if 1u64 << (tr * 8 + file) & block != 0 {break;}
    }
    for file in (0..=tf - 1).rev() {
        attacks |= 1u64 << (tr * 8 + file);
        if 1u64 << (tr * 8 + file) & block != 0 {break;}
    }

    attacks

}

// init leaper pieces attacks
//
//

pub fn init_pawn_attacks() -> [[u64 ; 64]; 2] {

    let mut pawn_attacks: [[u64 ; 64]; 2] = [[0u64; 64]; 2];

    for side in 0..2 {
        for square in 0..64 {
            pawn_attacks[side][square] = mask_pawn_attacks(square as u8, side as u8);
        }
    }

    pawn_attacks
}

pub fn init_knight_attacks() -> [u64 ; 64] {

    let mut knight_attacks: [u64 ; 64] = [0u64; 64];

    for square in 0..64 {
        knight_attacks[square] = mask_knight_attacks(square as u8);
    }

    knight_attacks
}

pub fn init_king_attacks() -> [u64 ; 64] {

    let mut king_attacks: [u64 ; 64] = [0u64; 64];

    for square in 0..64 {
        king_attacks[square] = mask_king_attacks(square as u8);
    }

    king_attacks
}

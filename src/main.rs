use std::time::{Instant};

mod bitboard;

// Main function
//
//

fn main() {
    println!("\n\n Welcome to Rusty Knight \n\n");

    let now = Instant::now();

    let pawn_attacks = bitboard::init_pawn_attacks();
    let knight_attacks = bitboard::init_knight_attacks();
    let king_attacks = bitboard::init_king_attacks();

    let square = bitboard::Squares::E5 as u8;
    let side = bitboard::Sides::White as u8;

    let mut block = 0u64;

    let block_square = bitboard::Squares::E3 as u8;
    set_bit!(block, block_square);

    let block_square = bitboard::Squares::G5 as u8;
    set_bit!(block, block_square);

    let block_square = bitboard::Squares::E7 as u8;
    set_bit!(block, block_square);

    let block_square = bitboard::Squares::C5 as u8;
    set_bit!(block, block_square);

    bitboard::print_bitboard(bitboard::rook_attacks_on_the_fly(square,block));

    println!("Runtime: {} μs", now.elapsed().as_micros());

}

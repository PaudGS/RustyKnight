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

    let square = bitboard::Squares::A1 as u8;
    let side = bitboard::Sides::White as u8;

    let mut attack_mask = bitboard::mask_rook_attacks(square);

    bitboard::set_occupancy(100, count_bits!(attack_mask), attack_mask);

    println!("Runtime: {} ns", now.elapsed().as_nanos());
}

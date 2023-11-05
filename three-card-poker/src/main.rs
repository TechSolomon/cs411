// main.rs
// Solomon Himelbloom
// 2023-10-27
//
// For CS 411 Fall 2023

use itertools::Itertools;

fn generate_hand() {
    // Generate every possible 3 card hand from a standard 52 card deck and determine which category the hand belongs to.

    let ranks = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];

    let suits = ['♠', '♥', '♦', '♣'];

    let mut deck = Vec::new();

    for rank in &ranks {
        for suit in &suits {
            deck.push(format!("{}{}", rank, suit));
        }
    }

    // Utilize permutations to generate every possible 3 card hand.

    let hands = deck.iter().permutations(3);

    let mut total_hands = 0;

    for _ in hands {
        total_hands += 1;
    }

    println!("Total hands: {}", total_hands);
}

fn payout_table() {
    // Once you have the probabilities, multiply by the payout and then sum for the total return.

    const STRAIGHT_FLUST: u32 = 100;
    const THREE_OF_A_KIND: u32 = 50;
    const STRAIGHT: u32 = 15;
    const FLUSH: u32 = 5;
    const PAIR: u32 = 1;
    const HIGH_CARD: u32 = 0;

    for payout in [
        STRAIGHT_FLUST,
        THREE_OF_A_KIND,
        STRAIGHT,
        FLUSH,
        PAIR,
        HIGH_CARD,
    ] {
        print!("{} ", payout);
    }
}

fn main() {
    let gui = "🃏";

    println!(">> Three Card Poker {}", gui);

    let bet = 1.00; // Assume a player makes a $1 bet and will get a payout for a pair or higher.

    let total_return = 0.00;

    generate_hand();

    payout_table();

    println!(
        "\n[ Current Bet: ${} | Total Return: ${} ]",
        bet, total_return
    );
}

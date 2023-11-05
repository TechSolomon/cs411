// main.rs
// Solomon Himelbloom
// 2023-10-27
//
// For CS 411 Fall 2023

use itertools::Itertools;
#[macro_use]
extern crate prettytable;
use prettytable::Table;

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

    // Utilize permutations & combinations to generate every possible 3 card hand.

    let hand_size = 3;

    let partial = deck.iter().permutations(hand_size);

    let hands = deck.iter().combinations(hand_size);

    // System Diagnostics

    println!(
        "With a deck of {} cards, there are {} permutations and {} combinations of {} card hands.",
        deck.len(),
        partial.clone().count(),
        hands.clone().count(),
        hand_size
    );
}

fn payout_table() {
    // Once you have the probabilities, multiply by the payout and then sum for the total return.

    const STRAIGHT_FLUST: u32 = 100;
    const THREE_OF_A_KIND: u32 = 50;
    const STRAIGHT: u32 = 15;
    const FLUSH: u32 = 5;
    const PAIR: u32 = 1;
    const HIGH_CARD: u32 = 0;

    let mut table = Table::new();

    table.add_row(row![
        "HAND",
        "DESCRIPTION",
        "FREQUENCY",
        "PROBABILITY",
        "PAYOUT",
        "RETURN"
    ]);
    table.add_row(row![
        "Straight Flush",
        "3 suited in sequence",
        "--",
        "--",
        STRAIGHT_FLUST,
        "--"
    ]);
    table.add_row(row![
        "Three of a Kind",
        "3 of the same rank",
        "--",
        "--",
        THREE_OF_A_KIND,
        "--"
    ]);
    table.add_row(row![
        "Straight",
        "3 in sequence (includes AKQ)",
        "--",
        "--",
        STRAIGHT,
        "--"
    ]);
    table.add_row(row!["Flush", "3 suited", "--", "--", FLUSH, "--"]);
    table.add_row(row!["Pair", "2 of the same rank", "--", "--", PAIR, "--"]);
    table.add_row(row![
        "High Card",
        "None of the above",
        "--",
        "--",
        HIGH_CARD,
        "--"
    ]);

    table.printstd();
}

fn main() {
    let gui = "🃏";

    println!(">> Three Card Poker {}", gui);

    let bet = 1.00; // Assume a player makes a $1 bet and will get a payout for a pair or higher.

    let total_return = 0.00;

    generate_hand();

    payout_table();

    println!(
        "Current Bet: ${}                                                               Total Return: ${}",
        bet, total_return
    );
}

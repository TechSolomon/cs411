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
    const THREE_OF_A_KIND: u32 = 30;
    const STRAIGHT: u32 = 15;
    const FLUSH: u32 = 5;
    const PAIR: u32 = 1;
    const HIGH_CARD: u32 = 0;

    let mut table = Table::new();

    // Frequency

    let first = 48;
    let second = 52;
    let third = 720;
    let fourth = 1096;
    let fifth = 3744;
    let sixth = 16440;

    // Probability

    let permutations = 22100;

    let prob1 = first as f32 / permutations as f32;
    let prob2 = second as f32 / permutations as f32;
    let prob3 = third as f32 / permutations as f32;
    let prob4 = fourth as f32 / permutations as f32;
    let prob5 = fifth as f32 / permutations as f32;
    let prob6 = sixth as f32 / permutations as f32;

    // Returns

    let ret1 = prob1 * STRAIGHT_FLUST as f32;
    let ret2 = prob2 * THREE_OF_A_KIND as f32;
    let ret3 = prob3 * STRAIGHT as f32;
    let ret4 = prob4 * FLUSH as f32;
    let ret5 = prob5 * PAIR as f32;
    let ret6 = prob6 * HIGH_CARD as f32;

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
        first,
        prob1,
        STRAIGHT_FLUST,
        ret1
    ]);
    table.add_row(row![
        "Three of a Kind",
        "3 of the same rank",
        second,
        prob2,
        THREE_OF_A_KIND,
        ret2
    ]);
    table.add_row(row![
        "Straight",
        "3 in sequence (includes AKQ)",
        third,
        prob3,
        STRAIGHT,
        ret3
    ]);
    table.add_row(row!["Flush", "3 suited", fourth, prob4, FLUSH, ret4]);
    table.add_row(row!["Pair", "2 of the same rank", fifth, prob5, PAIR, ret5]);
    table.add_row(row![
        "High Card",
        "None of the above",
        sixth,
        prob6,
        HIGH_CARD,
        ret6
    ]);

    table.printstd();

    let bet = 1.00; // Assume a player makes a $1 bet and will get a payout for a pair or higher.

    let total_return = ret1 + ret2 + ret3 + ret4 + ret5 + ret6;

    let total_return = (total_return * 100.0).round() / 100.0;

    println!(
        "Current Bet: ${}                                                                 Total Return: ${}",
        bet, total_return
    );
}

fn main() {
    let gui = "🃏";

    println!(">> Three Card Poker {}", gui);

    generate_hand();

    payout_table();
}

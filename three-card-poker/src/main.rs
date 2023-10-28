// main.rs
// Solomon Himelbloom
// 2023-10-27
//
// For CS 411 Fall 2023

fn main() {
    let gui = "🃏";
    let bet = 1.00;
    let total_return = 0.00;

    println!(">> Three Card Poker {}", gui);

    // https://en.wikipedia.org/wiki/Playing_cards_in_Unicode
    for suit in ['♠', '♥', '♦', '♣'].iter() {
        print!("{} ", suit);
    }

    println!(
        "\n[ Current Bet: ${} | Total Return: ${} ]",
        bet, total_return
    );
}

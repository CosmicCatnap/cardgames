// TERMS
// RANK - 2 through 10 then Joker, Queen, and King. The numeric value of a card
// SUIT - Spades, Hearts, Diamons, or Clubs. The symbol of the card

use std::{fmt, intrinsics::mir::Len};


// suites in american order ranking
#[derive(Copy, Clone)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

// define how we display suites when printed
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Suit::Spades => write!(f, "♠"),
           Suit::Hearts => write!(f, "♥"),
           Suit::Diamonds => write!(f, "♦"),
           Suit::Clubs => write!(f, "♣"),
       }
    }
}

#[derive(Copy, Clone)]
enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Value(u8),
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Rank::Ace => write!(f, "A"),
           Rank::King => write!(f, "K"),
           Rank::Queen => write!(f, "Q"),
           Rank::Jack => write!(f, "J"),
           Rank::Value(num) => write!(f, "{}", num.to_string()),
       }
    }
}

struct Card {
    suit: Suit,
    rank: Rank,
}

struct Deck {
    cards: Vec<Card>,
}

// In the lib
impl Default for Deck {
    fn default() -> Self {
        let mut cards = Vec::with_capacity(52);
        let Suits = [Suit::Spades, Suit::Diamonds, Suit::Hearts, Suit::Clubs];
        let Ranks = [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack];

        for &suit in Suits.iter() {
            for &rank in Ranks.iter() {
                cards.push(Card { suit, rank });
            }
            for rank in (2..11).map(|value| Rank::Value(value)) {
                cards.push(Card { suit, rank });
            }
        }
        
        Deck { cards }
    }
}

fn shuffle_deck(deck: Deck) {
    let passes = 3;
    let mut rng = thread_rng();
    rng.shuffle(deck.cards);
}

fn print_deck(deck: Deck) {
    for card in deck.cards {
        println!("{rank}{suit}", suit=card.suit, rank=card.rank);

        //print_card(card);
    }
}

fn print_card(card: Card) {
    println!("┌─────────┐");
    if card.rank.to_string() == "10" {
        println!("│ {rank}      │", rank=card.rank);
    } else {
        println!("│  {rank}      │", rank=card.rank);
    }
    println!("│         │");
    println!("│         │");
    println!("│    {suit}    │", suit=card.suit);
    println!("│         │");
    println!("│         │");
    if card.rank.to_string() == "10" {
        println!("│      {rank} │", rank=card.rank);
    } else {
        println!("│     {rank}   │", rank=card.rank);
    }
    println!("└─────────┘");
}

fn print_cards(cards: Vec<Card>) {
    let cardsize = cards.len();

    let mut topstring: String;
    for i in 0..cardsize {
        topstring.push('-');
    }
    print!("┌─────────┐");
    if card.rank.to_string() == "10" {
        println!("│ {rank}      │", rank=card.rank);
    } else {
        println!("│  {rank}      │", rank=card.rank);
    }
    println!("│         │");
    println!("│         │");
    println!("│    {suit}    │", suit=card.suit);
    println!("│         │");
    println!("│         │");
    if card.rank.to_string() == "10" {
        println!("│      {rank} │", rank=card.rank);
    } else {
        println!("│     {rank}   │", rank=card.rank);
    }
    println!("└─────────┘");
}
fn main() {
    let deck = Deck::default();

    println!("game started");
    print_deck(deck);

}

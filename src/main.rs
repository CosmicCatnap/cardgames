// TERMS
// RANK - 2 through 10 then Joker, Queen, and King. The numeric value of a card
// SUIT - Spades, Hearts, Diamons, or Clubs. The symbol of the card

use std::fmt;


// suites in american order ranking
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
        for &suit in &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for &rank in &[Rank::Ace, Rank::King, Rank::Queen, Rank::Jack] {
                cards.push(Card { suit, rank });
            }
            for rank in (2..11).map(|value| Rank::Value(value)) {
                cards.push(Card { suit, rank });
            }
        }
        
        Deck { cards }
    }
}
fn print_deck(deck: Deck) {

    for card in deck.cards {
        println!("{suit}{rank}", suit=card.suit, rank=card.rank);
    }
}

fn main() {
    let deck = Deck::default();

    println!("game started");
    print_deck(deck);

}

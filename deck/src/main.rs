
use rand::{thread_rng, seq::SliceRandom};

const NUM_DEAL: usize = 3;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self { // associated function
        Self { cards: Vec::new() }
    }

    fn add_card(&mut self, card: String) { // method
        self.cards.push(card);
    }

    fn fill_deck(&mut self) { // method
        let suits = ["hearts", "diamonds"];
        let ranks = ["ace", "two", "three"];

        for suit in suits {
            for rank in ranks {
                self.add_card(format!("{} of {}", rank, suit));
            }
        }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.fill_deck();

    println!("Here is your deck: {:?}", deck);

    deck.shuffle();
    println!("Here is your shuffled deck: {:?}", deck);

    if deck.cards.len() < NUM_DEAL {
        println!("Not enough cards to deal");
        return;
    }

    let hand = deck.deal(NUM_DEAL);
    println!("Here is your hand: {:?}", hand);
    println!("Here is your remaining deck: {:?}", deck);
}


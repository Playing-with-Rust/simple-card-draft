use crate::card::Card;
use crate::random::Random;
use std::fmt;
use crate::card::CardRarity;

#[derive(Debug, PartialEq)]
pub struct Pack {
    cards: Vec<Card>,
    is_open: bool
}

impl fmt::Display for Pack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_open {
            return Ok(())
        } 

        for card in &self.cards {
            write!(f, "{}", card)?;
        }

        Ok(())
    }
}

impl Pack {
    pub fn new() -> Self {
        Pack {
            cards: vec![
                Card::random(CardRarity::Common),
                Card::random(CardRarity::Common),
                Card::random(CardRarity::Common),
                Card::random(CardRarity::Common),
                Card::random(CardRarity::Uncommon),
                Card::random(CardRarity::Uncommon),
                Card::random(CardRarity::Rare),
            ],
            is_open: false
        }
    }

    pub fn open(&mut self) -> &Self {
        self.is_open = true;
        self
    }

    pub fn pick(&mut self, card_id: String) -> Option<Card> {
        if !self.is_open {
            println!("Pack is unopened");
            return None
        }

        let found_index = self.cards.iter().position(|x| *x.id == *card_id);

        match found_index {
            Some(index) => Some(self.cards.remove(index)),
            None => {
                println!("Found no card with indicated id");
                None
            },
        }
    }
}
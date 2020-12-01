use std::fmt;
use rand::Rng;
use rand::distributions::{Alphanumeric, Distribution, Standard};
use crate::random::Random;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CardKind {
    Asset,
    Proficiency,
    Ability
}

impl fmt::Display for CardKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CardKind::Asset => write!(f, "Asset"),
            CardKind::Proficiency => write!(f, "Proficiency"),
            CardKind::Ability => write!(f, "Ability")
        }
    }
}

impl Distribution<CardKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardKind {
        match rng.gen_range(0, 3) {
            0 => CardKind::Asset,
            1 => CardKind::Proficiency,
            _ => CardKind::Ability,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
}

impl Distribution<CardRarity> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardRarity {
        match rng.gen_range(0, 3) {
            0 => CardRarity::Common,
            1 => CardRarity::Uncommon,
            _ => CardRarity::Rare,
        }
    }
}

impl fmt::Display for CardRarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardRarity::Common => write!(f, "Common"),
            CardRarity::Uncommon => write!(f, "Uncommon"),
            CardRarity::Rare => write!(f, "Rare")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub kind: CardKind,
    pub rarity: CardRarity,
    pub description: String
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({id}) {rarity} Card: {name} - {kind}\n{description}\n\n",
            id = self.id,
            rarity = self.rarity,
            kind = self.kind,
            name = self.name,
            description = self.description
        )
    }
}

impl Random<Card, CardRarity> for Card {
    fn random(rarity: CardRarity) -> Card {
        let rng = rand::thread_rng();

        let id = rng
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect::<String>();

        let kind: CardKind = rand::random();

        use lipsum::MarkovChain;
        let mut chain_pre = MarkovChain::new_with_rng(rng);
        let mut chain = MarkovChain::new_with_rng(rng);
        chain_pre.learn("powerful mystic agile deceitful masterful trespassing eletric fiery");
        chain.learn("smash beam ball shot hit slash strike attack");

        let mut adjective = chain_pre.generate(1).to_string();
        let mut subjective =  chain.generate(1).to_string();
        &adjective.pop();
        &subjective.pop();
        let name = format!("{} {}", adjective, subjective).to_string();

        use lipsum::lipsum;
        let description = lipsum(25);

        let mut builder = CardBuilder::new();
            
        let builder = builder
            .set_id(id)
            .set_description(description)
            .set_kind(kind)
            .set_rarity(rarity)
            .set_name(name);
        
        builder
            .build()
            .unwrap()
    }
}

#[derive(Debug)]
pub enum CardBuilderError {
    IdMissing,
    NameMissing,
    KindMissing,
    RarityMissing,
    DescriptionMissing,
}

pub struct CardBuilder {
    id: Option<String>,
    name: Option<String>,
    kind: Option<CardKind>,
    rarity: Option<CardRarity>,
    description: Option<String>
}

impl CardBuilder {
    pub fn new () -> Self {
        CardBuilder {
            id: None,
            name: None,
            kind: None,
            rarity: None,
            description: None
        }
    }

    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn set_kind(&mut self, kind: CardKind) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    pub fn set_rarity(&mut self, rarity: CardRarity) -> &mut Self {
        self.rarity = Some(rarity);
        self
    }

    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn build(&self) -> Result<Card, CardBuilderError> {
        Ok(Card {
            id: Clone::clone(self.id.as_ref().ok_or(CardBuilderError::IdMissing)?),
            name: Clone::clone(self.name.as_ref().ok_or(CardBuilderError::NameMissing)?),
            kind: Clone::clone(self.kind.as_ref().ok_or(CardBuilderError::KindMissing)?),
            rarity: Clone::clone(self.rarity.as_ref().ok_or(CardBuilderError::RarityMissing)?),
            description: Clone::clone(self.description.as_ref().ok_or(CardBuilderError::DescriptionMissing)?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_chaining() {
        let id = "1".to_string();
        let description = "description".to_string();
        let name = "Test Card".to_string();

        let  card = CardBuilder::new()
            .set_id(id)
            .set_description(description)
            .set_kind(CardKind::Asset)
            .set_name(name)
            .build()
            .unwrap();

        assert_eq!(card.id, "1");
        assert_eq!(card.description, "description");
        assert_eq!(card.kind, CardKind::Asset);
        assert_eq!(card.name, "Test Card");
    }

    #[test]
    fn builder_non_chaining() {
        let id = "1".to_string();
        let description = "description".to_string();
        let name = "Test Card".to_string();

        let mut builder = CardBuilder::new();
            
        let builder = builder
            .set_id(id)
            .set_description(description)
            .set_kind(CardKind::Asset)
            .set_name(name);
        
        let card = builder
            .build()
            .unwrap();

        assert_eq!(card.id, "1");
        assert_eq!(card.description, "description");
        assert_eq!(card.kind, CardKind::Asset);
        assert_eq!(card.name, "Test Card");
    }
}
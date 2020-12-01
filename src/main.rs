mod card;
mod pack;
mod random;
mod helpers;

use card::Card;
use helpers::ask_input;
use pack::Pack;

fn main() {
    manual_flow()
}

fn manual_flow() {
    let mut picked_cards: Vec<Card> = vec!();
    
    let mut pack1 = Pack::new();
    pack1.open();
    picked_cards.push(pick_from_pack(pack1).unwrap());
    
    let mut pack2 = Pack::new();
    pack2.open();
    picked_cards.push(pick_from_pack(pack2).unwrap());
    
    let mut pack3 = Pack::new();
    pack3.open();
    picked_cards.push(pick_from_pack(pack3).unwrap());

    println!("Picked cards:\n{:?}", &picked_cards);
}

fn pick_from_pack(mut pack: Pack) -> Option<Card> {
    let input = ask_input(Some(format!("{}\n\nPlayer - Choose a card from this pack: ", pack)));
    println!("{}", input);

    pack.pick(input)
}
mod card;
mod pack;
mod random;
mod helpers;

use std::thread;
use std::sync::mpsc::channel;
use async_std::task;

async fn say_hello() {
    println!("Hello, world!");
}

fn manual_flow() {
    use crate::pack::Pack;
    use crate::helpers::ask_input;
    
    let mut pack = Pack::new();
    pack.open();

    let input = ask_input(Some(format!("{}\n\nPlayer 1 - Choose a card from the pack: ", pack)));
    println!("{}", input);
    let player_1_pick = pack.pick(input);
    println!("Picked: {}", player_1_pick.unwrap());
    
    let input = ask_input(Some(format!("{}\n\nPlayer 2 - Choose a card from the pack: ", pack)));
    let player_2_pick = pack.pick(input);
    println!("Picked: {}", player_2_pick.unwrap());

    println!("Pack has {} remaining...", pack.len())
}

fn main() {
    manual_flow()
}

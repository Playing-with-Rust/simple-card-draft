use std::io;

pub fn ask_input(question_text: Option<String>) -> String {
    if let Some(text) = question_text { println!("{}", text) }
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    
    let len = input.trim_end_matches(&['\r', '\n'][..]).len();
    input.truncate(len);
    
    input
}
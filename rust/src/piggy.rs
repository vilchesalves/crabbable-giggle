/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

use std::io;

fn capture_input(request: &str) -> String {
    let mut buf = String::from("");

    println!("{}", request);

    match io::stdin().read_line(&mut buf) {
        Ok(_) => buf,
        Err(e) => {
            println!("Error! {}", e);
            "Can't read".to_string()
        }
    }
}

fn is_vowel(s: &char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    vowels.contains(s)
}

fn piggy_word(word: &str) -> String {
    let first_char = word.chars().nth(0).unwrap();
    let mut piggy = String::from("");

    if is_vowel(&first_char) {
        piggy.push(first_char);
        piggy.push_str("-hay");
    } else {
        let without_first_char = &word[1..];

        piggy.push_str(without_first_char);
        piggy.push_str("-");
        piggy.push(first_char);
        piggy.push_str("ay");
    }

    piggy
}

fn to_pig_latin(input: &str) -> String {
    let iter = input.split_whitespace();
    let mut pig = String::from("");

    for word in iter {
        let pigged = piggy_word(word);
        pig.push_str(" ");
        pig.push_str(&pigged);
    }

    pig.truncate(pig.trim_end().len());
    pig
}

fn main() {
    let user_input = capture_input("Please input!");
    let user_pig = to_pig_latin(&user_input);

    println!("read: {}", user_input);
    println!("pig: {}", user_pig);
}

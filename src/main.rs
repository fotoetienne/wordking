use std::fs;
use std::io;

fn main() {
    println!("Provide your guesses thus far");
    let mut guess_n = 0;
    // letters we know are not in the word
    let mut invalid_letters = String::new();
    // letters we know are in the word
    let mut valid_letters = String::new();
    // the letters that we know where they go
    let mut known_letters = "     ".to_string().chars().collect::<Vec<_>>();
    // for each slot, a list of chars that we know exist but are in the wrong place
    let mut wrong_place = vec![
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ];

    let mut collect_guess = || -> bool {
        println!("Guess {}:", guess_n);
        guess_n += 1;
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let chars = guess.split_whitespace().collect::<Vec<_>>();

        if chars.len() != 5 {
            if !chars.len() == 0 {
                println!("Words are only 5 letters");
            }
            return false;
        }

        for (i, c) in chars.iter().enumerate() {
            if i >= 5 {
                println!("Words are only 5 letters");
                break;
            }
            let value = c.chars().next().unwrap();
            if c.contains("!") {
                if !valid_letters.contains(value) {
                    valid_letters.push(value)
                }
                known_letters[i] = value;
            } else if c.contains("?") {
                if !valid_letters.contains(value) {
                    valid_letters.push(value)
                }
                wrong_place[i].push(value)
            } else if !invalid_letters.contains(value) {
                invalid_letters.push(value);
            }
        }

        true
    };

    let mut keep_going = true;
    while keep_going {
        keep_going = collect_guess();
    }

    let known_letters = String::from_iter(known_letters);

    println!("Valid letters: {}", valid_letters);
    println!("Invalid letters: {}", invalid_letters);
    println!("Known letters: {}", known_letters);

    let without_invalids = without_invalid_letter(get_words(), invalid_letters);
    let with_valids = with_valid_letters(without_invalids, valid_letters);
    let without_misplaced = with_known_letters(with_valids, known_letters);
    let filtered = without_misplaced_letters(without_misplaced, wrong_place);

    let suggestion = filtered.join("\n");

    println!("Try: {}", suggestion);
}

fn get_words() -> Vec<String> {
    fs::read_to_string("words.txt")
        .expect("Unable to load word list")
        .split_whitespace()
        .map(|word| word.to_string())
        .collect()
}

// Filter out words that contain invalid letters
fn without_invalid_letter(words: Vec<String>, invalid_letters: String) -> Vec<String> {
    words.into_iter().filter(
        |word| invalid_letters
            .chars()
            .all(|letter| !word.contains(letter))
    )
        .collect::<Vec<String>>()
}

// Filter out words that don't contain known valid letters
fn with_valid_letters(words: Vec<String>, valid_letters: String) -> Vec<String> {
    words.into_iter().filter(
        |word| valid_letters
            .chars()
            .all(|letter| word.contains(letter))
    )
        .collect::<Vec<String>>()
}

// Filter out words that don't contain known letters in the right place
fn with_known_letters(words: Vec<String>, known_letters: String) -> Vec<String> {
    words.into_iter().filter(
        |word| known_letters.chars().zip(word.chars())
            .all(|(known_letter, guessed_letter)|
                known_letter == ' ' || guessed_letter == known_letter)
    )
        .collect::<Vec<String>>()
}

// Filter out words that contain in the wrong place
fn without_misplaced_letters(words: Vec<String>, wrong_place: Vec<String>) -> Vec<String> {
    words.into_iter().filter(
        |word| wrong_place.iter().zip(word.chars())
            .all(|(invalid_letters, guessed_letter)|
                !invalid_letters.contains(guessed_letter))
    )
        .collect::<Vec<String>>()
}

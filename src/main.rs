use std::fs;
use std::io;
use itertools::Itertools;

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
    let game_context = GameContext { invalid_letters, valid_letters, known_letters, wrong_place };
    // let filtered = filter_words(get_words(), game_context);
    // let suggestion = filtered.join("\n");
    let ranked_guesses = rank_guesses(game_context);
    let suggestion = ranked_guesses.iter()
        .map(|(score, guess)|
            guess.to_owned() + " "  + &*score.to_string()
        )
        .join("\n");

    // println!("Valid letters: {}", valid_letters);
    // println!("Invalid letters: {}", invalid_letters);
    // println!("Known letters: {}", known_letters);
    println!("Try: {}", suggestion);
}

#[derive(Clone)]
struct GameContext {
    invalid_letters: String,
    // letters we know are in the word
    valid_letters: String,
    // the letters that we know where they go
    known_letters: String,
    // for each slot, a list of chars that we know exist but are in the wrong place
    wrong_place: Vec<String>,
}

fn get_words() -> Vec<String> {
    fs::read_to_string("wordle_short_list.txt")
        .expect("Unable to load word list")
        .split_whitespace()
        .map(|word| word.to_string())
        .collect()
}

fn filter_words(words: Vec<String>, game_context: GameContext) -> Vec<String> {
    let without_invalids = without_invalid_letter(words, game_context.invalid_letters);
    let with_valids = with_valid_letters(without_invalids, game_context.valid_letters);
    let without_misplaced = with_known_letters(with_valids, game_context.known_letters);
    return without_misplaced_letters(without_misplaced, game_context.wrong_place);
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

fn rank_guesses(game_context: GameContext) -> Vec<(usize, String)> {
    let options = filter_words(get_words(), game_context.clone());
    options.iter().map(|guess|
        (score_guess(guess.to_string(), game_context.clone()), guess.to_string())
    ).sorted().collect::<Vec<_>>()
}

// The guess score is the average of the possible options remaining
fn score_guess(guess: String, game_context: GameContext) -> usize {
    let options = filter_words(get_words(), game_context.clone());
    return options.iter().map(|actual|
        simulate_guess(actual.to_string(), guess.clone(), game_context.clone())
    ).sum::<usize>();
}

// Given <actual> solution word, if <guess> is used, how many <options> remain?
fn simulate_guess(actual: String, guess: String, game_context: GameContext) -> usize {
    let post_guess_context = make_guess(actual, guess, game_context.clone());
    let options = filter_words(get_words(), post_guess_context);
    options.len()
}

// Update game_context based on a guess
fn make_guess(actual: String, guess: String, mut game_context: GameContext) -> GameContext {
    let mut known_letters = game_context.known_letters.chars().collect::<Vec<_>>();

    for (i, (guess_char, actual_char)) in guess.chars().zip(actual.chars()).enumerate() {
        if actual.contains(guess_char) {
            if !game_context.valid_letters.contains(guess_char) {
                game_context.valid_letters.push(guess_char);
            }
            if guess_char == actual_char {
                known_letters[i] = guess_char;
            }
        } else if !game_context.invalid_letters.contains(guess_char) {
            game_context.invalid_letters.push(guess_char);
        }
    }

    game_context.known_letters = String::from_iter(known_letters);
    game_context
}

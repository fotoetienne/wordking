# Wordking

A simple cli tool to help with [wordle](https://www.powerlanguage.co.uk/wordle/) in hard mode.

# Usage

Run wordking

```bash
cargo run
```

Wordking will ask for your guesses thus far.

```bash
Provide your guesses thus far
Guess 0:
```

Provide the five letters separated by spaces. For a letter that is in the correct place (green), follow it with a
bang (!). For a letter that is correct but misplaced, follow it with a question mark (?). Press enter when you've
entered all of your guesses.

```bash
Provide your guesses thus far
Guess 0:
t? e r? a s
Guess 1:
f r? u i t!

Valid letters: tr
Invalid letters: easfui
Known letters:     t
Try: robot
```

Wordking will provide a list of words to try next that incorporate the feedback on the words you've guessed thus far.

```
Wordle 215 3/6*

🟨⬛🟨⬛⬛
⬛🟨⬛⬛🟩
🟩🟩🟩🟩🟩
```
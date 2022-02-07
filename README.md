# wordle in your terminal

## Wordle

[Wordle](https://www.powerlanguage.co.uk/wordle/) is a word guessing game.
Each day you have to guess a new word.
You have 6 attempts and each attempt you make reveals some information.

![Example](assets/example.png)

## Usage

```sh
# play the daily wordle game
wordle

# play the given day (eg if you missed a day)
wordle day <day>

# play a random game
wordle random

# play a custom word
wordle custom <word>
```

## Install

```sh
cargo install cl-wordle --locked
```

## Demo

![Demo](assets/demo.gif)

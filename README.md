# wordle in your terminal

[![download](https://img.shields.io/badge/dynamic/json?color=brightgreen&label=downloads&query=%24.crate.downloads&url=https%3A%2F%2Fcrates.io%2Fapi%2Fv1%2Fcrates%2Fcl-wordle&style=flat-square)](https://crates.io/crates/cl-wordle)
[![docs](https://img.shields.io/docsrs/cl-wordle?style=flat-square)](https://docs.rs/cl-wordle/latest/cl_wordle/)

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
wordle day <day-number>

# play the given date
wordle date <year-month-day>

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

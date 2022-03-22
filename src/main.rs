//! This program solves "bulls and cows" game as fast as possible.



mod direct_struct_access_denier {

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct Number {
        a: i8,
        b: i8,
        c: i8,
        d: i8,
    }

    impl Number {
        pub const fn new(a: i8, b: i8, c: i8, d: i8) -> Number {
            assert!(Number::is_correct(a, b, c, d));
            Number { a, b, c, d }
        }

        pub fn from(n: &str) -> Number {
            assert!(n.len() == 4);
            let a: i8 = (n.as_bytes()[0] as char).to_string().parse::<i8>().unwrap();
            let b: i8 = (n.as_bytes()[1] as char).to_string().parse::<i8>().unwrap();
            let c: i8 = (n.as_bytes()[2] as char).to_string().parse::<i8>().unwrap();
            let d: i8 = (n.as_bytes()[3] as char).to_string().parse::<i8>().unwrap();
            Number::new(a, b, c, d)
        }

        pub const fn is_correct(a: i8, b: i8, c: i8, d: i8) -> bool {
            (0 <= a && a <= 9) &&
            (0 <= b && b <= 9) &&
            (0 <= c && c <= 9) &&
            (0 <= d && d <= 9) &&
            (a != b) &&
            (a != c) &&
            (a != d) &&
            (b != c) &&
            (b != d) &&
            (c != d)
        }

        pub fn to_string(self) -> String {
            let (a, b, c, d) = self.to_tuple();
            format!("{a}{b}{c}{d}")
        }

        pub const fn to_tuple(self) -> (i8, i8, i8, i8) {
            (self.a, self.b, self.c, self.d)
        }
    }



    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct BullsAndCows {
        bulls: i8,
        cows: i8,
    }

    impl BullsAndCows {
        pub const fn get_bulls(self) -> i8 { self.bulls }
        // pub const fn get_cows(self) -> i8 { self.cows }

        pub const fn new(bulls: i8, cows: i8) -> BullsAndCows {
            assert!(BullsAndCows::is_correct(bulls, cows));
            BullsAndCows { bulls, cows }
        }

        pub const fn is_correct(bulls: i8, cows: i8) -> bool {
            (0 <= bulls && bulls <= 4) &&
            (0 <= cows && cows <= 4) &&
            (0 <= bulls+cows && bulls+cows <= 4)
        }

        pub const fn from(n: Number, m: Number) -> BullsAndCows {
            let bulls: i8 =
                (n.a == m.a) as i8 +
                (n.b == m.b) as i8 +
                (n.c == m.c) as i8 +
                (n.d == m.d) as i8;
            let cows: i8 =
                (n.a == m.b) as i8 +
                (n.a == m.c) as i8 +
                (n.a == m.d) as i8 +
                (n.b == m.a) as i8 +
                (n.b == m.c) as i8 +
                (n.b == m.d) as i8 +
                (n.c == m.a) as i8 +
                (n.c == m.b) as i8 +
                (n.c == m.d) as i8 +
                (n.d == m.a) as i8 +
                (n.d == m.b) as i8 +
                (n.d == m.c) as i8;
            BullsAndCows::new(bulls, cows)
        }

        pub const fn sum(self) -> i8 { self.bulls + self.cows }
    }



    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct Guess {
        number: Number,
        bulls_and_cows: BullsAndCows,
    }

    impl Guess {
        pub const fn get_number(self) -> Number { self.number }
        pub const fn get_bulls_and_cows(self) -> BullsAndCows { self.bulls_and_cows }

        pub const fn new(number: Number, bulls_and_cows: BullsAndCows) -> Guess {
            Guess { number, bulls_and_cows }
        }
    }



}



use std::{str::FromStr, fmt::Debug};
use std::io::{stdout, Write};

use rand::Rng;
// use itertools::Itertools;
use rand::{prelude::SliceRandom, thread_rng};

pub mod all_legal_numbers;
use all_legal_numbers::ALL_LEGAL_NUMBERS;

use direct_struct_access_denier::*;



trait ExcludeAll<T> {
    fn exclude(self, items_to_exclude: Vec<T>) -> Vec<T>;
}

impl<T> ExcludeAll<T> for Vec<T>
where T: Copy + std::cmp::PartialEq
{
    fn exclude(self, items_to_exclude: Vec<T>) -> Vec<T> {
        let mut res: Vec<T> = self;
        // items_to_exclude.iter().for_each(|&item_to_exclude| {
        //     let index_to_remove: usize = res.iter().position(|&digit| digit == item_to_exclude).unwrap();
        //     res.remove(index_to_remove);
        // });
        for item_to_exclude in items_to_exclude {
            let index_to_remove: usize = res.iter().position(|&digit| digit == item_to_exclude).unwrap();
            res.remove(index_to_remove);
        }
        res
    }
}



type Guesses = Vec<Guess>;



fn random<T>(min: T, max: T) -> T
where T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform
{
    rand::thread_rng().gen_range(min..max)
}

// fn random_digit() -> i8 {
//     rand::thread_rng().gen_range(0..10)
// }

const DIGITS: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn generate_number_random(exclude_digits: Vec<i8>) -> Number {
    let mut rng = thread_rng();
    let mut digits: Vec<i8> = DIGITS.to_vec().exclude(exclude_digits.clone());
    digits.shuffle(&mut rng);
    let a = digits.pop().unwrap();
    let b = digits.pop().unwrap();
    let (c, d) = if !digits.is_empty() {
        (digits.pop().unwrap(), digits.pop().unwrap())
    }
    else {
        digits.extend(exclude_digits);
        digits.shuffle(&mut rng);
        (digits.pop().unwrap(), digits.pop().unwrap())
    };
    Number::new(a, b, c, d)
}

fn generate_number_smart(guesses: &Guesses) -> Option<Number> {
    let currently_legal_numbers: Vec<Number> = ALL_LEGAL_NUMBERS.into_iter()
        .filter(|&number|
            guesses.iter().all(|guess|
                guess.get_bulls_and_cows() == BullsAndCows::from(number, guess.get_number())
            )
        )
        .collect();
    if IS_PLAYER {
        currently_legal_numbers.iter().for_each(|n| print!("{ns} ", ns = n.to_string()));
        println!();
    }
    if !currently_legal_numbers.is_empty() {
        // TODO: maybe optimize here:
        Some(currently_legal_numbers[random(0, currently_legal_numbers.len())])
    }
    else {
        None
    }
}



/// Generates number to guess or returns None, if it is impossible
fn generate_number_to_guess(guesses: &Guesses) -> Option<Number> {
    // TODO: maybe optimize here:
    match guesses.len() {
        0 => {
            // Some(generate_number_random(vec![]))
            Some(Number::from("0123"))
        }
        // 1 => {
        //     let (a, b, c, d) = guesses[0].get_number().to_tuple();
        //     Some(generate_random_number(vec![a, b, c, d]))
        //     // Some(generate_random_number(vec![]))
        //     // Some(Number::from("3456"))
        // }
        // 2 => {
        //     // TODO: maybe optimize here
        //     // let g0: Guess = guesses[0];
        //     // let g1: Guess = guesses[1];
        //     // let (a, b, c, d) = g0.get_number().to_tuple();
        //     // let (e, f, g, h) = g1.get_number().to_tuple();
        //     // let used_digits: Vec<i8> = vec![a, b, c, d, e, f, g, h];
        //     // if g0.get_bulls_and_cows().sum() + g1.get_bulls_and_cows().sum() < 4 {
        //     //     Some(generate_random_number(used_digits))
        //     // }
        //     // else {
        //     //     Some(generate_random_number(DIGITS.to_vec().exclude(used_digits)))
        //     // }
        //     Some(generate_random_number(vec![]))
        //     // Some(Number::from("6789"))
        // }
        // 3 => {
        //     Some(generate_random_number(vec![]))
        // }
        _ => {
            generate_number_smart(guesses)
        }
    }
}



#[allow(unused_must_use)]
fn prompt<T: FromStr>(text: &str) -> T where <T as FromStr>::Err: Debug {
    print!("{text}");
    stdout().flush();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer.trim().parse::<T>().unwrap()
}



/// returns number of guesses used to finish
fn play_game(number_of_player: Option<Number>) -> u8 {
    // let is_player: bool = number_of_player.is_none();
    assert_eq!(IS_PLAYER, number_of_player.is_none());
    let mut guesses: Guesses = vec![];

    let mut n: u8 = 0;
    loop {
        n += 1;
        let number_to_guess = match generate_number_to_guess(&guesses) {
            Some(number_to_guess) => { number_to_guess }
            None => {
                if IS_PLAYER {
                    println!("You inputed something wrong. Exiting...");
                }
                break;
            }
        };
        if IS_PLAYER {
            println!("My guess: {number_to_guess}", number_to_guess = number_to_guess.to_string());
        }
        let bulls_and_cows: BullsAndCows = if IS_PLAYER {
            let bulls: i8 = prompt("Bulls: ");
            let cows : i8 = prompt("Cows : ");
            BullsAndCows::new(bulls, cows)
        }
        else {
            BullsAndCows::from(number_to_guess, number_of_player.unwrap()) 
        };
        if bulls_and_cows.get_bulls() == 4 {
            if IS_PLAYER {
                println!("Answer is {number}, guessed in {n}.", number = number_to_guess.to_string());
            }
            break;
        }
        guesses.push(Guess::new(number_to_guess, bulls_and_cows));
    }
    n
}



#[cfg(not(feature = "is_bench"))] const IS_PLAYER: bool = true;
#[cfg(    feature = "is_bench") ] const IS_PLAYER: bool = false;

fn main() {
    if IS_PLAYER {
        play_game(None);
    }
    else {
        let ns: [u8; 5040] = bench();
        let avg: f64 = ns.into_iter().map(|n: u8| n as u64).sum::<u64>() as f64 / ns.len() as f64;
        println!("avg = {avg}");
    }
}





fn count_guesses(number: Number) -> u8 {
    play_game(Some(number))
}

fn bench() -> [u8; 5040] {
    assert_eq!(5040, ALL_LEGAL_NUMBERS.len());
    let mut res: [u8; 5040] = [0; 5040];
    for i in 0..5040 {
        let number: Number = ALL_LEGAL_NUMBERS[i];
        res[i] = count_guesses(number);
    }
    res
}





#[cfg(test)]
mod tests {
    use crate::ExcludeAll;

    #[test]
    fn exclude() {
        assert_eq!(
            vec![1, 4, 5, 7, 8],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].exclude(vec![0, 2, 3, 6, 9])
        );
    }
}




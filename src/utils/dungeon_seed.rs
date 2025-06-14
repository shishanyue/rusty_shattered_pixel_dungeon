use anyhow::Result;
use bevy::prelude::*;
use radix_fmt::radix;
//This class defines the parameters for seeds in ShatteredPD and contains a few convenience methods

//larges possible seed has a value of 26^9
pub const TOTAL_SEEDS: i64 = 5429503678976;

#[derive(Default,Resource)]
pub struct DugeonSeed;
pub type Seed = i64;

impl DugeonSeed {
    //Seed codes take the form @@@-@@@-@@@ where @ is any letter from A to Z (only uppercase)
    //This is effectively a base-26 number system, therefore 26^9 unique seeds are possible.

    //Seed codes exist to make sharing and inputting seeds easier
    //ZZZ-ZZZ-ZZZ is much easier to enter and share than 5,429,503,678,975

    //generates a random seed, omitting seeds that contain vowels (to minimize real words appearing randomly)
    //This means that there are 21^9 = 794,280,046,581 unique seeds that can be randomly generated
    pub fn random_seed() -> Seed {
        let mut seed;
        loop {
            seed = rand::random_range(0..=TOTAL_SEEDS);

            if let Ok(result) = Self::convert_to_code(seed) {
                if result.contains('A')
                    || result.contains('E')
                    || result.contains("I")
                    || result.contains("O")
                    || result.contains("U")
                {
                    break seed;
                }
            }
        }
    }
    //Takes a long value and converts it to the equivalent seed code
    pub fn convert_to_code(seed: i64) -> Result<String> {
        if !(0..TOTAL_SEEDS).contains(&seed) {
            panic!("seeds must be within the range [0, TOTAL_SEEDS)");
        }
        //this almost gives us the right answer, but its 0-p instead of A-Z
        let interrim = radix(seed, 26).to_string();
        let mut result = String::new();
        for i in 0..9 {
            if i < interrim.len() {
                if let Some(mut c) = interrim.chars().nth(i) {
                    if c <= '9' {
                        c = (c as u8 + 17) as char;
                    } else {
                        c = (c as u8 - 22) as char;
                    }

                    result.push(c);
                }
            } else {
                result.insert(0, 'A');
            }
        }

        //insert dashes for readability
        result.insert(3, '-');
        result.insert(7, '-');

        Ok(result)
    }

    //Takes a seed code (@@@@@@@@@) and converts it to the equivalent long value
    pub fn convert_from_code(code: &str) -> Result<Seed> {
        //if code is formatted properly, force uppercase
        let code = if code.len() == 11
            && code.chars().nth(3) == Some('-')
            && code.chars().nth(7) == Some('-')
        {
            code.to_uppercase()
        } else {
            code.to_string()
        };
        if code.len() != 9 {
            panic!("codes must be 9 A-Z characters.");
        }

        let mut result = 0;

        for i in (0..8).rev() {
            let c = code.chars().nth(i).unwrap();
            if !c.is_ascii_uppercase() {
                panic!("codes must be 9 A-Z characters.");
            }

            result += ((c as u8 - 65) * 26_u8.pow((8 - i) as u32)) as i64;
        }
        Ok(result)
    }

    fn format_text(input_text: &str) -> String {
        if let Ok(result) = Self::convert_from_code(input_text){
            if let Ok(result) = Self::convert_to_code(result) {
                result
            }else {
                input_text.to_string()
            }
        }else {
            input_text.to_string()
        }
    }
}

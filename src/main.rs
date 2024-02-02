use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use rand::distributions::{Distribution, WeightedIndex};

fn main() {
    let mut buf = String::new();

    File::open("./names.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    // I want to create next character prediction
    // * step 1 create biogram based prediction (only consider prev character)
    //  - what is accurance of specific character pair?

    // For following list of words:
    // emma
    // naria
    // jan
    // michal
    //
    // and considering only neighbor charatecters (biogram), the following engrams can be computed
    // em - 1
    // mm - 1
    // ma - 1
    // na - 1
    // ar - 1
    // ri - 1
    // ia - 1
    // ja - 1
    // an - 1
    // mi - 1
    // ic - 1
    // ch - 1
    // ha - 1
    // al - 1
    //
    // This means that each character pair has the same likely hood of accurence
    // But wait we need some special character to indicate start and end of the word. Lets add them to prev list
    // ...
    // .e - 1
    // .j - 1
    // .m - 1
    // .n - 1
    // a. - 2 it accurs in `emma` and `naria`
    // l. - 1
    // n. - 1

    // Now lets randomly (but considering accurence propability) choose pair that starts with `.`
    // .e - 1
    // .j - 1
    // .m - 1
    // .n - 1
    // There are 4 equally accuring starting combinations so each of them has 25% chance to accur.
    // Formula for computing chance is 1 / (1 + 1 + 1 + 1) = 1 / 4 = 0.25

    // `.m` has been drawn.

    // Repeat previous step but choose pairs that starts with `m`

    // Now lets randomly (but considering accurence propability) choose pair that starts with `m`
    // mm - 1
    // ma - 1
    // mi - 1
    // There are 3 equally accuring starting combinations so each of them has 33.(3)% chance to accur.

    // and so on

    let mut all_characters: HashSet<char> = HashSet::from_iter(buf.chars());
    all_characters.remove(&'\n');
    const START_END_CHARACTER: char = '`'; // next characters are lowercase alphabeth letters: a, b, c...
    all_characters.insert(START_END_CHARACTER);
    let mut char_to_index = HashMap::new();
    let mut index_to_char = HashMap::new();
    for c in all_characters {
        let index = (c as u8 - START_END_CHARACTER as u8) as usize;
        char_to_index.insert(c, index);
        index_to_char.insert(index, c);
    }

    let row = (0..char_to_index.len()).map(|_| 0_u32).collect::<Vec<_>>();

    let mut engrams = (0..char_to_index.len())
        .map(|_| row.clone())
        .collect::<Vec<_>>();

    for word in buf.lines() {
        for (p, n) in Some(START_END_CHARACTER)
            .into_iter()
            .chain(word.chars())
            .zip(word.chars().chain(Some(START_END_CHARACTER)))
        {
            engrams[*char_to_index.get(&p).unwrap()][*char_to_index.get(&n).unwrap()] += 1;
        }
    }

    let engram_distribution_model = engrams
        .iter()
        .map(|row| WeightedIndex::new(row).unwrap())
        .collect::<Vec<_>>();

    let mut rng = thread_rng();
    for _ in 0..100 {
        let mut character_index = *char_to_index.get(&START_END_CHARACTER).unwrap();
        loop {
            character_index = engram_distribution_model[character_index].sample(&mut rng);
            let character = *index_to_char.get(&character_index).unwrap();

            if character == START_END_CHARACTER {
                break;
            };

            print!("{character}");
        }

        println!("");
    }

    // Compute how well model performs for given word
}

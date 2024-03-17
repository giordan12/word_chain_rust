// This program will generate a word chain where every word starts with
// the later that the previous word ended with. The prorgam will output
// the longest chain and its length. If there are multiple word chains,
// then we can output any of them.

// Plan of action:
// we only care about the first and last letter of the words, so we will extract
// those into a list of tuples. We will start with the first tuple and count 0, we will look at its
// last letter and come up with a list of options without taking into account the first
// tuple, right now the count is increased by 1. Then we will go on each one of those options
// and we will repeat the process again. When we run out of options we will return the chain
// length and we will compare vs what has been returned from all the other options. The max value
// will keep climbing back up.

fn main() {
    let input_words = vec![
        "apple", "elephant", "tiger", "rabbit", "cat", "dog", "giraffe", "egg", "goat", "tiger",
        "rat", "turtle",
    ];

    let mut word_elements: Vec<WordElements> = Vec::new();
    for word in &input_words {
        word_elements.push(WordElements::from_word(word));
    }

    println!("{:?}", word_elements);

    let mut d: u8 = 0;

    let w = word_elements.remove(0);

    let max_word_chain = get_max_word_chain(w, word_elements, d);
    print!("Result is {}", max_word_chain);
}

#[derive(Debug)]
struct WordElements {
    first_letter: char,
    last_letter: char,
}

impl WordElements {
    fn from_word(word: &str) -> Self {
        // todo we need to make sure that the word is at least 2 chars long here and maybe not empty
        let first_char = word
            .chars()
            .nth(0)
            .expect("There was no first char in the word");
        let last_char = word
            .chars()
            .last()
            .expect("There was no last char in the word");
        WordElements {
            first_letter: first_char,
            last_letter: last_char,
        }
    }
}

impl Clone for WordElements {
    fn clone(&self) -> Self {
        WordElements {
            first_letter: self.first_letter,
            last_letter: self.last_letter,
        }
    }
}

fn get_max_word_chain(
    current_word: WordElements,
    word_vector: Vec<WordElements>,
    mut depth: u8,
) -> u8 {
    // end of recursion: we cannot go any further so we return the current depth

    if word_vector.len() == 0 {
        println!("Found all possible word chain for this path");
        return depth;
    }

    depth += 1;

    println!("current word: {current_word:?}");
    println!("word vector: {:?}", word_vector);
    println!("depth is {}", depth);
    println!("");

    // here we will find the new WordElement to continue the chain and we will increase the depth

    let current_last_letter = current_word.last_letter;
    // here we need to search for a word in word_vector that has a first_letter that matches the current_last_letter

    let mut branch_depth: Vec<u8> = Vec::new();
    let mut found_match = false;
    for (
        i,
        WordElements {
            first_letter: new_first_letter,
            last_letter: _,
        },
    ) in word_vector.iter().enumerate()
    {
        if (*new_first_letter == current_last_letter) {
            // we need to keep recursing here passing the new last letter and removing the
            // entry from the vector

            let mut new_word_vector = word_vector.to_vec();
            let new_word = new_word_vector.remove(i);

            let new_depth = get_max_word_chain(new_word, new_word_vector, depth);

            // we will add the depth that we get from this set of paths to the total
            branch_depth.push(new_depth);
            found_match = true;
        }
    }

    if found_match {
        match branch_depth.iter().max() {
            Some(max) => return *max,
            None => panic!("Why couldn't I find a max value?"),
        }
    }

    // there is no word to containue the word chain, we need to return the current depth
    //

    println!("Found no more possible combinations\n");
    depth
    // println!("Adding current max depth to the overall depth");
    // depth += current_max_depth;
    // depth
}

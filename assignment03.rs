// Assignment 3 - Word count with a HashMap

use std::collections::HashMap;

// 1. count_words: how many times each word appears.
// Words are separated by whitespace, case does not matter.
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let lower = word.to_lowercase();
        // How many times have we seen this word so far? 0 if never.
        let old = match counts.get(&lower) {
            Some(n) => *n,
            None => 0,
        };
        counts.insert(lower, old + 1);
    }
    counts
}

// 2. count_of: how many times one word appears (0 if it is not in the map).
fn count_of(counts: &HashMap<String, usize>, word: &str) -> usize {
    let lower = word.to_lowercase();
    match counts.get(&lower) {
        Some(n) => *n,
        None => 0,
    }
}

// 3. total_words: the sum of all counts.
fn total_words(counts: &HashMap<String, usize>) -> usize {
    let mut total = 0;
    for n in counts.values() {
        total += *n;
    }
    total
}

// 4. sorted_words: most common first, equal counts in alphabetical order.
fn sorted_words(counts: &HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut list: Vec<(String, usize)> = Vec::new();
    for (word, n) in counts {
        list.push((word.clone(), *n));
    }
    // First sort alphabetically.
    list.sort();
    // Then sort by count, biggest first. This sort keeps the order of equal
    // counts, so they stay alphabetical.
    list.sort_by(|a, b| b.1.cmp(&a.1));
    list
}

fn main() {
    let counts = count_words("the cat The dog THE end");
    println!("{}", count_of(&counts, "the"));
    println!("{}", count_of(&counts, "horse"));
    println!("{}", total_words(&counts));
    println!("{:?}", sorted_words(&counts));

    let counts2 = count_words("Rust is fun and Rust is fast");
    println!("{}", total_words(&counts2));
    println!("{:?}", sorted_words(&counts2));

    let empty = count_words("");
    println!("{}", total_words(&empty));
    println!("{:?}", sorted_words(&empty));
}

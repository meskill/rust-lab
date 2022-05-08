use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn get_sorted_vec(word: &str) -> Vec<String> {
    let mut sorted_word: Vec<String> = word.graphemes(true).map(String::from).collect();

    sorted_word.sort_unstable();

    sorted_word
}

pub fn anagrams_for<'entry>(word: &str, possible_anagrams: &[&'entry str]) -> HashSet<&'entry str> {
    let word = word.to_lowercase();
    let sorted_word = get_sorted_vec(&word);

    possible_anagrams
        .iter()
        .filter(|&&entry| entry.len() == word.len())
        .filter(|entry| {
            let entry = entry.to_lowercase();

            entry != word && get_sorted_vec(&entry) == sorted_word
        })
        .copied()
        .collect()
}

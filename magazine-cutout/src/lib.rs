// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::collections::HashMap;
pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    //! Returns true if the note can be constructed from the magazine.
    //! The note is a list of words, and the magazine is a list of words.
    //! Magazine words can only be used once.
    if magazine.len() < note.len() {
        return false;
    }
    
    let magazine_map = magazine.iter().fold(HashMap::new(), |mut words, str| {
        *words.entry(str).or_insert(0) += 1;
        words
    });
    let note_map = note.iter().fold(magazine_map, |mut words, str| {
        *words.entry(str).or_insert(0) -= 1;
        words
    });
    
    // Return true if magazine map has values >= 0,
    // meaning all words in the note were found in the magazine.
    note_map.values().all(|&count| count >= 0)
}
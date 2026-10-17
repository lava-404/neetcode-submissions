use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut word_map: HashMap<[i32; 26], Vec<String>> = HashMap::new();
        for word in strs{
            let mut counts = [0; 26];
            for ch in word.chars(){
                counts[(ch as u8 - b'a') as usize] += 1;
            }
            word_map.entry(counts).or_insert_with(Vec::new).push(word);
        }
          word_map.into_values().collect()
    }
    
    /*pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // For each word, we generate the frequency key.
        // Before pushing the word into the output vec, we check if its
        // frequency key matches an existing one.
        // If it does, we push it into the value vec for that key in the HashMap.
        // Repeat for all words.
        // Return the output vec.

        let mut word_map: HashMap<String, Vec<String>> = HashMap::new();

        for word in strs {
            let freq_key: String = Self::create_freq_key(&word);
            word_map.entry(freq_key).or_insert(vec![]).push(word);
        }

        word_map.into_values().collect()
    }

    pub fn create_freq_key(word: &str) -> String {
        let mut freq_map = HashMap::new();

        for ch in word.chars() {
            *freq_map.entry(ch).or_insert(0) += 1;
        }

        let mut entries: Vec<_> = freq_map.into_iter().collect();
        entries.sort_by_key(|&(ch, _)| ch);

        entries
            .iter()
            .map(|(ch, count)| format!("{}:{}", ch, count))
            .collect::<Vec<_>>()
            .join(",")
    }*/
}
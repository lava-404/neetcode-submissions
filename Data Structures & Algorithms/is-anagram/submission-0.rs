use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        //create 2 hashmaps
        //iterate over the strings and build the hashmap
        //if hashmaps match, return true

        let mut s_hashmap: HashMap<char, i32> = HashMap::new();
        let mut t_hashmap: HashMap<char, i32> = HashMap::new();
        for char in s.chars(){
            *s_hashmap.entry(char).or_insert(0) += 1;
        }
        for char in t.chars(){
            *t_hashmap.entry(char).or_insert(0) += 1;
        }

        s_hashmap == t_hashmap
    }
}

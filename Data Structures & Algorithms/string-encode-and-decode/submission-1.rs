impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut final_string = String::from("");

        for word in strs {
            let len = word.len();
            final_string.push_str(&len.to_string());
            final_string.push('#');
            final_string.push_str(&word); // missing this
        }

        final_string
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut word_vec: Vec<String> = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut len = String::from("");

            while bytes[i] != b'#' {
                len.push(bytes[i] as char);
                i += 1;
            }

            let len_num: usize = len.parse().unwrap();
            i += 1; // skip '#'

            let mut new_word = String::from("");

            for _ in 0..len_num {
                new_word.push(bytes[i] as char);
                i += 1;
            }

            word_vec.push(new_word);
        }

        word_vec
    }
}
impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded = String::new();

        for s in strs {
            encoded.push_str(&s.len().to_string());
            encoded.push('#');
            encoded.push_str(&s);
        }

        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut result = Vec::new();
        let mut i = 0;

        while i < bytes.len() {
            let mut len = 0;

            while bytes[i] != b'#' {
                len = len * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }

            i += 1; // Skip '#'
            let end = i + len;

            result.push(s[i..end].to_string());
            i = end;
        }

        result
    }
}
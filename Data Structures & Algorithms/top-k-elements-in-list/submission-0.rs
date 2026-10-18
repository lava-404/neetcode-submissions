use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_map = HashMap::new();
        let mut output: Vec<i32> = Vec::new();

        for num in nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        let mut max_value: i32 = 0;
        let mut max_key: i32 = 0;
        let mut i = 0;
        while i < k {
            for (key, value) in freq_map.iter(){
                if *value > max_value {
                    max_value = *value;
                    max_key = *key;
                }
            }
            freq_map.retain(|&key, _| &key != &max_key);
            output.push(max_key);
            i += 1;
            max_value = 0;
        }
        output
    }
}

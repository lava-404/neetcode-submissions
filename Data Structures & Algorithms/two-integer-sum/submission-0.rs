impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //create a hashmap storing i32 i32
        //insert number and its complement
        //check if complement already exists in array
        //return indices of number and the complement

        let mut indice_map = HashMap::new();
        let mut output: Vec<i32> = Vec::new();
        let mut indice: i32 = 0;

        //target is 9 and num is 7
        for num in nums {
            let complement = target - num;
            if let Some(&position) = indice_map.get(&complement) {
                output.push(position);
                output.push(indice);
                return output;
            }
            else {
                indice_map.insert(num, indice);
            }
            indice += 1;
        }
        output
    }
}

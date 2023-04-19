use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut compliments: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            match compliments.get(&nums[i]) {
                Some(&x) => return vec![x, i as i32],
                None => compliments.insert(target - nums[i], i as i32),
            };
        }
        return vec![-1,-1];
    }
}
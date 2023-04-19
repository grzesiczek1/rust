// This function takes in a vector of integers and a target integer
// It returns a vector of two indices whose elements add up to the target
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a HashMap to store compliments of each element
        let mut compliments: HashMap<i32, i32> = HashMap::new();
        
        // Iterate through the vector
        for i in 0..nums.len() {
            // Check if the compliment of the current element exists in the HashMap
            match compliments.get(&nums[i]) {
                // If it does, return the indices of the two elements
                Some(&x) => return vec![x, i as i32],
                // Otherwise, insert the compliment of the current element into the HashMap
                None => compliments.insert(target - nums[i], i as i32),
            };
        }
        // Return [-1,-1] if no two elements add up to the target
        return vec![-1,-1];
    }
}
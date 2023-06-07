use std::collections::HashMap;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ret_arr: Vec<i32> = vec![1; nums.len()];
        let mut right = 1;
        let mut left = 1;

        for i in 0..nums.len() {
            ret_arr[i] *= left;
            left *= nums[i];
        }
        
        for i in (0..nums.len()).rev() {
            ret_arr[i] *= right;
            right *= nums[i];
        }
        

        ret_arr
    }
}
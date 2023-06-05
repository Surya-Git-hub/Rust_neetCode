impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let numlen = nums.len() as i32;



        loop {
            if i >= numlen {
                return vec![];
            }

            for j in (i + 1)..numlen {
                if j >= numlen {
                    return vec![];
                }

                if nums[i as usize] + nums[j as usize] == target {
                    return vec![i, j];
                }
            }

            i += 1;
        }
    }
}

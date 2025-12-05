struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        if sum % k == 0 {
            return 0;
        } else {
            return sum % k;
        }
    }
}

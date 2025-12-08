// Category: algorithms
// Level: Easy
// Percent: 89.68899%

// Given an array of integers nums, return the number of good pairs.
//
// A pair (i, j) is called good if nums[i] == nums[j] and i < j.
//
//
// Example 1:
//
// Input: nums = [1,2,3,1,1,3]
// Output: 4
// Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
//
//
// Example 2:
//
// Input: nums = [1,1,1,1]
// Output: 6
// Explanation: Each pair in the array are good.
//
//
// Example 3:
//
// Input: nums = [1,2,3]
// Output: 0
//
//
//
// Constraints:
//
//
// 	1 <= nums.length <= 100
// 	1 <= nums[i] <= 100
//

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut freq = [0i32; 101];

        for &x in &nums {
            freq[x as usize] += 1;
        }

        let mut good = 0i32;
        for &cnt in freq.iter() {
            if cnt > 1 {
                good += cnt * (cnt - 1) / 2;
            }
        }

        good
    }
}

// Category: algorithms
// Level: Easy
// Percent: 91.46411%



// You are given a string s. The score of a string is defined as the sum of the absolute difference between the ASCII values of adjacent characters.
//
// Return the score of s.
//
//  
// Example 1:
//
//
// Input: s = "hello"
//
// Output: 13
//
// Explanation:
//
// The ASCII values of the characters in s are: 'h' = 104, 'e' = 101, 'l' = 108, 'o' = 111. So, the score of s would be |104 - 101| + |101 - 108| + |108 - 108| + |108 - 111| = 3 + 7 + 0 + 3 = 13.
//
//
// Example 2:
//
//
// Input: s = "zaz"
//
// Output: 50
//
// Explanation:
//
// The ASCII values of the characters in s are: 'z' = 122, 'a' = 97. So, the score of s would be |122 - 97| + |97 - 122| = 25 + 25 = 50.
//
//
//  
// Constraints:
//
//
// 	2 <= s.length <= 100
// 	s consists only of lowercase English letters.
//

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut sum: i32 = 0;
        let mut prev_char: Option<u32> = None;
        for char in s.chars() {
            let ascii_value = u32::from(char);
            if prev_char != None {
            sum += (prev_char.unwrap() as i32 - ascii_value as i32).abs();
            }
            prev_char = Some(ascii_value);
        }
        sum
    }
}

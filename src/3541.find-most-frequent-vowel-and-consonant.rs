// Category: algorithms
// Level: Easy
// Percent: 89.65555%



// You are given a string s consisting of lowercase English letters ('a' to 'z'). 
// 
// Your task is to:
// 
// 
// 	Find the vowel (one of 'a', 'e', 'i', 'o', or 'u') with the maximum frequency.
// 	Find the consonant (all other letters excluding vowels) with the maximum frequency.
// 
// 
// Return the sum of the two frequencies.
// 
// Note: If multiple vowels or consonants have the same maximum frequency, you may choose any one of them. If there are no vowels or no consonants in the string, consider their frequency as 0.
// The frequency of a letter x is the number of times it occurs in the string.
//  
// Example 1:
// 
// 
// Input: s = "successes"
// 
// Output: 6
// 
// Explanation:
// 
// 
// 	The vowels are: 'u' (frequency 1), 'e' (frequency 2). The maximum frequency is 2.
// 	The consonants are: 's' (frequency 4), 'c' (frequency 2). The maximum frequency is 4.
// 	The output is 2 + 4 = 6.
// 
// 
// 
// Example 2:
// 
// 
// Input: s = "aeiaeia"
// 
// Output: 3
// 
// Explanation:
// 
// 
// 	The vowels are: 'a' (frequency 3), 'e' ( frequency 2), 'i' (frequency 2). The maximum frequency is 3.
// 	There are no consonants in s. Hence, maximum consonant frequency = 0.
// 	The output is 3 + 0 = 3.
// 
// 
// 
//  
// Constraints:
// 
// 
// 	1 <= s.length <= 100
// 	s consists of lowercase English letters only.
// 
 

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut vowel_count = [0; 5]; // a, e, i, o, u
        let mut consonant_count = [0; 21]; // 21 consonants in English
        
        for ch in s.chars() {
            match ch {
                'a' => vowel_count[0] += 1,
                'e' => vowel_count[1] += 1,
                'i' => vowel_count[2] += 1,
                'o' => vowel_count[3] += 1,
                'u' => vowel_count[4] += 1,
                _ => {
                    // Map consonants to indices: b=0, c=1, d=2, ..., z=20
                    let index = ch as usize - 'b' as usize;
                    if index < 21 {
                        consonant_count[index] += 1;
                    }
                }
            }
        }
        
        let max_vowel = vowel_count.iter().max().copied().unwrap_or(0);
        let max_consonant = consonant_count.iter().max().copied().unwrap_or(0);
        
        max_vowel + max_consonant
    }
}

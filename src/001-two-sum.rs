use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut new_map: HashMap<i32, i32> = HashMap::new();

        for (i, j) in nums.iter().enumerate() {
            let i = i as i32;

            let residue = target - j;
            let residue_index = *new_map.get(&residue).unwrap_or(&-1);

            if new_map.contains_key(&residue) && residue_index != i {
                return vec![i, residue_index];
            }
            new_map.insert(*j, i);
        }

        return vec![0, 0];
    }
}

/*
+------------------------------------+
| No need for the leetcode           |
+------------------------------------+
*/

struct Solution;

fn main() {
    let a = Solution::two_sum(vec![3, 3], 6);
    println!("{:?}", a)
}

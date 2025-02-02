use std::collections::HashMap;

fn solution(nums: Vec<i32>, target: i32) -> (i32, i32) {
    let mut new_map: HashMap<i32, i32> = HashMap::new();

    for (i, j) in nums.iter().enumerate() {
        let i = i as i32;

        new_map.insert(*j, i);

        let residue = target - j;

        if new_map.contains_key(&residue) {
            return (i, *new_map.get(&residue).unwrap());
        }
    }

    return (0, 0);
}

fn main() {
    let (a, b) = solution(vec![1, 2, 3], 5);
    println!("{a} --> {b}")
}

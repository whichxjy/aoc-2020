use std::cmp::Ordering;

fn solve_part_one(nums: &[u64]) -> u64 {
    fn is_valid_num(nums: &[u64], idx: usize) -> bool {
        let mut preamble = nums[idx - 25..idx].to_vec();
        preamble.sort_unstable();

        let mut left = 0;
        let mut right = preamble.len() - 1;

        while left < right {
            match nums[idx].cmp(&(preamble[left] + preamble[right])) {
                Ordering::Less => right -= 1,
                Ordering::Greater => left += 1,
                Ordering::Equal => return true,
            }
        }

        false
    }

    let invalid_idx = (25..nums.len())
        .find(|idx| !is_valid_num(nums, *idx))
        .unwrap();

    nums[invalid_idx]
}

fn main() {
    let content = include_str!("../input.txt");
    let nums = content
        .trim()
        .split('\n')
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    assert_eq!(solve_part_one(&nums), 373803594);
}

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

fn solve_part_two(nums: &[u64]) -> u64 {
    fn find_range(nums: &[u64], target: u64) -> Option<(usize, usize)> {
        // Get prefix sum of nums.
        // i == 0: prefix_sum[i] = 0
        // i >= 1: prefix_sum[i] = nums[0] + ... + nums[i - 1]
        let mut prefix_sum = vec![0; nums.len() + 1];
        prefix_sum[0] = nums[0];
        for i in 1..prefix_sum.len() {
            prefix_sum[i] = nums[i - 1] + prefix_sum[i - 1];
        }

        for left in 0..nums.len() {
            for right in left..nums.len() {
                let range_sum = prefix_sum[right + 1] - prefix_sum[left];

                if range_sum == target {
                    return Some((left, right));
                }
            }
        }

        None
    }

    let target = solve_part_one(&nums);
    let (left, right) = find_range(nums, target).unwrap();

    let max_num = nums[left..=right].iter().max().unwrap();
    let min_num = nums[left..=right].iter().min().unwrap();

    max_num + min_num
}

fn main() {
    let content = include_str!("../input.txt");
    let nums = content
        .trim()
        .split('\n')
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    assert_eq!(solve_part_one(&nums), 373803594);
    assert_eq!(solve_part_two(&nums), 51152360);
}

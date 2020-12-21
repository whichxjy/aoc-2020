use std::cmp::Ordering;

fn solve_part_one(nums: &[u64]) -> u64 {
    fn is_valid_num(preamble: &[u64], num: u64) -> bool {
        let mut left = 0;
        let mut right = preamble.len() - 1;

        while left < right {
            match num.cmp(&(preamble[left] + preamble[right])) {
                Ordering::Less => right -= 1,
                Ordering::Greater => left += 1,
                Ordering::Equal => return true,
            }
        }

        false
    }

    let mut preamble = nums[..25].to_vec();
    preamble.sort_unstable();

    *nums
        .iter()
        .skip(25)
        .find(|num| is_valid_num(&preamble, **num))
        .unwrap()
}

fn main() {
    let content = include_str!("../input.txt");
    let nums = content
        .trim()
        .split('\n')
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    assert_eq!(solve_part_one(&nums), 0);
}

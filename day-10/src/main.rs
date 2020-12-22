fn solve_part_one(nums: &[u32]) -> u32 {
    let mut nums = nums.to_vec();
    nums.sort_unstable();

    let mut one_jolt_count = 0;
    let mut three_jolts_count = 1;

    let mut last_rating = 0;

    for num in nums {
        let diff = num - last_rating;
        last_rating = num;

        if diff < 1 || diff > 3 {
            continue;
        }

        if diff == 1 {
            one_jolt_count += 1;
        }

        if diff == 3 {
            three_jolts_count += 1;
        }
    }

    one_jolt_count * three_jolts_count
}

fn main() {
    let content = include_str!("../input.txt");
    let nums = content
        .trim()
        .split('\n')
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    assert_eq!(solve_part_one(&nums), 1904);
}

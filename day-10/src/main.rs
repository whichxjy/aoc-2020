use std::collections::HashMap;

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

fn solve_part_two(nums: &[u32]) -> u64 {
    let mut nums = nums.to_vec();
    nums.sort_unstable();

    // num -> ways
    let mut ways_count: HashMap<u32, u64> = HashMap::new();
    ways_count.insert(0, 1);

    let steps = vec![1, 2, 3];

    for num in &nums {
        for step in &steps {
            if num < step {
                continue;
            }

            if let Some(prev_ways) = ways_count.get(&(num - step)) {
                let new_count = ways_count.get(&num).unwrap_or(&0) + prev_ways;
                ways_count.insert(*num, new_count);
            }
        }
    }

    *ways_count.get(nums.last().unwrap()).unwrap()
}

fn main() {
    let content = include_str!("../input.txt");
    let nums = content
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    assert_eq!(solve_part_one(&nums), 1904);
    assert_eq!(solve_part_two(&nums), 10578455953408);
}

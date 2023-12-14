pub fn run() {
    let input = include_str!("../../inputs/day09.txt");
    let (mut res, mut res_part2) = (0, 0);

    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap());

        let mut derivate: Vec<i32> = nums.clone().collect();

        for part in 1..=2 {
            if part == 2 {
                derivate = nums.clone().collect();
                derivate.reverse();
            }

            let mut derivates: Vec<Vec<i32>> = vec![derivate.clone()];

            while derivate.iter().any(|x| *x != 0) {
                derivate = sub(&derivate);
                derivates.push(derivate.clone());
            }

            derivates.iter().rev().for_each(|x| {
                if part == 1 {
                    res += x.last().unwrap();
                } else {
                    res_part2 += x.last().unwrap();
                }
            });
        }
    }

    println!("Part 1 : {}, part 2 : {}", res, res_part2);
}

fn sub(nums: &Vec<i32>) -> Vec<i32> {
    let mut new_nums = nums.clone();
    for i in 1..nums.len() {
        new_nums[i] -= nums[i - 1];
    }
    new_nums.remove(0);
    new_nums
}

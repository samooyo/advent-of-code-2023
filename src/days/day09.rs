pub fn run() {
    let input = include_str!("../../inputs/day09.txt");
    let mut res = 0;
    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap());

        let mut derivate: Vec<i32> = nums.clone().collect();
        let mut derivates: Vec<Vec<i32>> = vec![derivate.clone()];

        while derivate.iter().any(|x| *x != 0) {
            derivate = derivatee(&derivate);
            derivates.push(derivate.clone());
        }
        derivates.iter().rev().for_each(|x| {
            res += x.last().unwrap();
        });
    }

    println!("Result : {}", res);
}

fn derivatee(nums: &Vec<i32>) -> Vec<i32> {
    let mut new_nums = nums.clone();
    for i in 1..nums.len() {
        new_nums[i] -= nums[i - 1];
    }
    new_nums.remove(0);
    new_nums
}

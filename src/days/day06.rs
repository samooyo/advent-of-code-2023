pub fn run() {
    let input = include_str!("../../inputs/day06_part2.txt");

    let (time, distance) = input.split_once('\n').unwrap();
    let clean_time = time.split_once(": ").unwrap().1;
    let clean_distance = distance.split_once(": ").unwrap().1;

    let time_numbers = clean_time
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let distance_numbers = clean_distance
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut glob_wins = 1;
    for race_number in 0..time_numbers.len() {
        let mut wins = 0;
        let time = time_numbers[race_number];
        let distance = distance_numbers[race_number];

        for i in 1..time {
            let res = i * (time - i);
            if res > distance {
                wins += 1;
            }
					}
        glob_wins *= wins;
    }
    println!("Day 6: part1 = {}", glob_wins)
}

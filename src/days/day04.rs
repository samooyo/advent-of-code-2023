pub fn run() {
    let input = include_str!("../../inputs/day04.txt");

    let mut scratchcards_array = [1u32; 194];
    let mut glob_points = 0;

    for (line_number, line) in input.lines().enumerate() {
        let (full_card, full_win) = line.split_once('|').unwrap();
        let (_, clean_card) = full_card.split_once(':').unwrap();

        let card_numbers: Vec<&str> = clean_card.split_whitespace().collect();
        let winning_numbers: Vec<&str> = full_win.split_whitespace().collect();

        let mut good_numbers = 0;
        let mut points = 0;
        for num in winning_numbers {
            if card_numbers.contains(&num) {
                good_numbers += 1;
                points = if points == 0 { 1 } else { points * 2 };
            }
        }

        glob_points += points;

        for i in 1..=good_numbers {
            scratchcards_array[i + line_number] += scratchcards_array[line_number];
        }
    }
    let mut res = 0;
    scratchcards_array.iter().for_each(|x| res += x);

    println!("Day 4: part1 = {}, part2 = {}", glob_points, res);
}

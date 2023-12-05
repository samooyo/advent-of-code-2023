pub fn run() {
    let my_str = include_str!("../../inputs/day01.txt");

    let mut count_p1 = 0;
    let mut count_p2 = 0;

    let string_num = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in my_str.lines() {
        let first_digit_pos = line.find(|c: char| c.is_ascii_digit());
        let second_digit_pos = line.rfind(|c: char| c.is_ascii_digit());

        let (mut first_digit_p2, mut second_digit_p2) = (None, None);
        let mut first_digit_pos_p2 = if first_digit_pos.is_some() {
            first_digit_pos
        } else {
            None
        };
        let mut second_digit_pos_p2 = if second_digit_pos.is_some() {
            second_digit_pos
        } else {
            None
        };

        for (i, number) in string_num.iter().enumerate() {
            let pos = line.find(number);
            if pos.is_some() {
                if first_digit_pos_p2.is_none() || pos.unwrap() < first_digit_pos_p2.unwrap() {
                    first_digit_p2 = Some(i + 1);
                    first_digit_pos_p2 = pos;
                }
                if second_digit_pos_p2.is_none()
                    || line.rfind(number).unwrap() > second_digit_pos_p2.unwrap()
                {
                    second_digit_p2 = Some(i + 1);
                    second_digit_pos_p2 = line.rfind(number);
                }
            }
        }

        let mut res = 0;
        let mut decimal = 0;
        let mut unit = 0;

        if first_digit_pos.is_some() {
            decimal = (line.as_bytes()[first_digit_pos.unwrap()] as char)
                .to_digit(10)
                .unwrap();
            unit = (line.as_bytes()[second_digit_pos.unwrap()] as char)
                .to_digit(10)
                .unwrap();

            res = decimal * 10 + unit;

            if first_digit_pos_p2 < first_digit_pos {
                decimal = first_digit_p2.unwrap() as u32;
            }
            if second_digit_pos_p2 > second_digit_pos {
                unit = second_digit_p2.unwrap() as u32;
            }
        } else {
            if first_digit_p2.is_some() {
                decimal = first_digit_p2.unwrap() as u32;
            }
            if second_digit_p2.is_some() {
                unit = second_digit_p2.unwrap() as u32;
            }
        }

        count_p1 += res;
        count_p2 += decimal * 10 + unit;
    }
    println!("And the answer for part1 is {}", count_p1);
    println!("And the answer for part2 is {}", count_p2);
}

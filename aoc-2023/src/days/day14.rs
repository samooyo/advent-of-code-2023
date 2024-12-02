pub fn run() {
    let input = include_str!("../../inputs/day14.txt");

    let map: Vec<&str> = input.split_whitespace().collect();

    let mut res = 0;
    map.iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == 'O' {
                let mut o_counter = 0;
                for row in (0..i).rev() {
                    if map[row].chars().nth(j).unwrap() == 'O' {
                        o_counter += 1;
                    } else if map[row].chars().nth(j).unwrap() == '#' {
                        o_counter += row + 1;
                        break;
                    }
                }
                res += map.len() - o_counter;
            }
        });
    });

    println!("Part1 : {}", res);
}

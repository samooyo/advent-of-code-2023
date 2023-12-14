pub fn run() {
    let input = include_str!("../../inputs/test.txt");

    let mut positions: Vec<(usize, usize)> = vec![];
    let mut empty_horizontal = vec![];
    let mut empty_vertical = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        if line.contains(|x: char| x.is_alphabetic()) {
            empty_horizontal.push(y);
        }
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                positions.push((x, y));
            }
        });
    });

    let lines = input.lines();
    for (y, line) in lines.enumerate() {
        if line.contains(|x: char| x.is_alphabetic()) {
            empty_vertical.push(y);
        }
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                positions.push((x, y));
            }
        });
    }

    println!("{:?}", positions)
}

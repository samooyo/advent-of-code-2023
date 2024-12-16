pub fn run() {
    let input = include_str!("../../inputs/day10.txt");

    let map: Vec<&str> = input.split_whitespace().collect();
    let mut y = map.iter().position(|&x| x.contains('S')).unwrap();
    let mut x = map[y].find('S').unwrap();
    let (mut prev_x, mut prev_y) = (x, y);
    (x, y) = find_beginning(map.clone(), x, y);

    let mut steps = 0;
    while map[y].chars().nth(x).unwrap() != 'S' {
        match map[y].chars().nth(x).unwrap() {
            '-' => {
                if prev_x == x - 1 {
                    prev_x = x;
                    x += 1
                } else {
                    prev_x = x;
                    x -= 1
                };
                prev_y = y;
            }
            '|' => {
                if prev_y == y - 1 {
                    prev_y = y;
                    y += 1
                } else {
                    prev_y = y;
                    y -= 1
                };
                prev_x = x;
            }
            '7' => {
                if prev_x == x - 1 {
                    prev_y = y;
                    y += 1;
                    prev_x = x;
                } else {
                    prev_x = x;
                    x -= 1;
                    prev_y = y;
                };
            }
            'J' => {
                if prev_y == y - 1 {
                    prev_x = x;
                    x -= 1;
                    prev_y = y;
                } else {
                    prev_y = y;
                    y -= 1;
                    prev_x = x;
                };
            }
            'L' => {
                if prev_x == x + 1 {
                    prev_y = y;
                    y -= 1;
                    prev_x = x;
                } else {
                    prev_x = x;
                    x += 1;
                    prev_y = y;
                };
            }
            'F' => {
                if prev_y == y + 1 {
                    prev_x = x;
                    x += 1;
                    prev_y = y;
                } else {
                    prev_y = y;
                    y += 1;
                    prev_x = x;
                };
            }
            _ => panic!("Wrong char"),
        }
        steps += 1;
    }

    let new_map: Vec<&str> = map.clone();
    for line in map.iter() {
        for (i, char) in line.chars().enumerate() {
            if char == '.' {
                let mut counter = 0;
                for x in i + 1..line.len() {
                    if "-|JFL7".contains(line.chars().nth(x).unwrap()) {
                        counter += 1;
                    }
                }
                println!("{}", counter);
                if counter % 2 == 0 {
                    println!("not inside");
                } else if line.contains('F') && line.contains('7') {
                    println!("not inside special");
                } else {
                    line.to_string().replace_range(i..i + 1, "O");
                    println!("{}", line);
                    println!("inside")
                }
            }
        }
    }
    for line in new_map.iter() {
        println!("{}", line);
    }
    println!("Max distance = {}", steps / 2 + 1);
}

fn find_beginning(map: Vec<&str>, x: usize, y: usize) -> (usize, usize) {
    if "-J7".contains(map[y].chars().nth(x + 1).unwrap()) {
        (x + 1, y)
    } else if "-FL".contains(map[y].chars().nth(x - 1).unwrap()) {
        (x - 1, y)
    } else if "|F7".contains(map[y - 1].chars().nth(x).unwrap()) {
        (x, y - 1)
    } else if "|JL".contains(map[y + 1].chars().nth(x).unwrap()) {
        (x, y + 1)
    } else {
        panic!("Wrong map")
    }
}

// fn inside_loop() {}

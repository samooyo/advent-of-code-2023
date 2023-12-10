pub fn run() {
    let input = include_str!("../../inputs/test.txt");

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

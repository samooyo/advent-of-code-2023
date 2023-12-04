pub fn run() {
    let (max_red, max_green, max_blue) = (12, 13, 14);
    let (mut total_p1, mut total_p2) = (0, 0);

    let my_str = include_str!("../../inputs/day02.txt");
    let lines = my_str.lines();

    for line in lines {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        let (game_number, data) = line.split_once(": ").unwrap();
        let game = data.split("; ");

        game.for_each(|x| {
            for cube in x.split(", ") {
                let number = cube.split(' ').next().unwrap().parse::<usize>().unwrap();
                let color = cube.split(' ').nth(1).unwrap();

                if color == "red" {
                    red = red.max(number);
                } else if color == "green" {
                    green = green.max(number);
                } else if color == "blue" {
                    blue = blue.max(number);
                } else {
                    panic!("Wrong color")
                }
            }
        });
        if (red <= max_red) && (green <= max_green) && (blue <= max_blue) {
            total_p1 += (game_number.split_once(' ').unwrap().1)
                .parse::<usize>()
                .unwrap();
        }
        total_p2 += red * green * blue;
    }
    println!("part1 : {}, part2 : {}", total_p1, total_p2);
}

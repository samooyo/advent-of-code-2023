pub fn run() {
    let my_str = include_str!("../../inputs/day05.txt");
    let binding = my_str.to_string();

    let mut almanac = vec![];

    let (seeds, maps) = binding.split_once("\n\n").unwrap();
    let seeds = seeds.replace("seeds:", "");

    let seeds_part1: Vec<usize> = seeds
        .replace("seeds:", "")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut seeds_part2 = vec![];
    for (i, seed) in seeds_part1.iter().enumerate() {
        if i % 2 == 0 {
            for j in 0..seeds_part1[i + 1] {
                seeds_part2.push(seed + j);
            }
        }
    }

    let categories: Vec<&str> = maps.split("\n\n").collect();

    for category in categories {
        let mut instr = vec![];

        for line in category.lines().skip(1) {
            let [destiantion, source, range] = line
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();
            instr.push([destiantion, source, range]);
        }
        almanac.push(instr);
    }

    let mut res: Vec<usize> = vec![];
    for seed in seeds_part1 {
        let mut location = seed;
        for category in &almanac {
            let mut tmp_seed = location;
            for &[destination, source, range] in category {
                if (source..(source + range)).contains(&tmp_seed) {
                    tmp_seed = tmp_seed - source + destination;
                    break;
                }
            }
            location = tmp_seed;
        }
        res.push(location);
    }
    println!("part1 : {}", res.iter().min().unwrap());

    let mut res: Vec<usize> = vec![];
    for seed in seeds_part2 {
        let mut location = seed;
        for category in &almanac {
            let mut tmp_seed = location;
            for &[destination, source, range] in category {
                if (source..(source + range)).contains(&tmp_seed) {
                    tmp_seed = tmp_seed - source + destination;
                    break;
                }
            }
            location = tmp_seed;
        }
        res.push(location);
    }
    println!("part2 : {}", res.iter().min().unwrap());
}

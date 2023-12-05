pub fn run() {
    let my_str = include_str!("../../inputs/day05.txt");
    let binding = my_str.to_string();

    let mut almanac = vec![];

    let (seeds, maps) = binding.split_once("\n\n").unwrap();
    let seeds = seeds.replace("seeds:", "");

    let clean_seeds: Vec<usize> = seeds
        .replace("seeds:", "")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
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
    for seed in clean_seeds {
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
}

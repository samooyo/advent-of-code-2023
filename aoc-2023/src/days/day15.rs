pub use indexmap::{IndexMap, IndexSet};

pub fn run() {
    let input = include_str!("../../inputs/day15.txt");

    let map: Vec<&str> = input.split(',').collect();
    let mut boxes: Vec<IndexMap<String, usize>> = vec![IndexMap::new(); 256];
    let mut part1 = 0;

    map.iter().for_each(|set| {
        let mut current_value = 0;
        let mut box_number = 0;

        set.chars().for_each(|c| {
            if c == '-' || c == '=' {
                box_number = current_value;
            }
            current_value += c as u32;
            current_value *= 17;
            current_value %= 256;
        });
        part1 += current_value;

        if set.contains('=') {
            let (label, value) = set.split_once('=').unwrap();
            boxes[box_number as usize].insert(label.to_string(), value.parse::<usize>().unwrap());
        } else {
            let (label, _) = set.split_once('-').unwrap();
            boxes[box_number as usize].shift_remove(label);
        }
    });

    let mut part2 = 0;
    boxes.iter().enumerate().for_each(|(i, boxx)| {
        boxx.iter().enumerate().for_each(|(j, (_, value))| {
            part2 += (1 + i) * (1 + j) * value;
        });
    });

    println!("Part1 : {}, part2 : {}", part1, part2);
}

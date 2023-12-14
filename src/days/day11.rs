pub fn run() {
    let input = include_str!("../../inputs/test.txt");

    let mut new_map: Vec<String> = vec![];
    let map: Vec<&str> = input.split_whitespace().collect();

    map.iter().enumerate().for_each(|(i, line)| {
        if line.contains('#') {
            new_map.push(line.to_string());
        } else {
            new_map.push(line.to_string());
            new_map.push(line.to_string());
        }
    });

    new_map.iter().for_each(|line| println!("{}", line));
}

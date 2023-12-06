pub struct Match {
    pub time: i64,
    pub distance: i64,
}

fn parse_values(line: &String) -> Vec<i64> {
    line.split_once(':')
        .unwrap().1.split(' ')
        .filter(|field| !field.is_empty())
        .map(|field| field.parse::<i64>().unwrap())
        .collect()
}

pub fn parse_input(lines: &Vec<String>) -> Vec<Match> {
    let time_values: Vec<i64> = parse_values(&lines[0]);
    let distance_values: Vec<i64> = parse_values(&lines[1]);

    time_values.into_iter().zip(distance_values)
        .map(|(time, distance)| Match { time: time, distance: distance }).collect()
}

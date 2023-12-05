use std::ops::Range;

pub struct RangeFunction {
    pub source_range: Range<i64>,
    pub desintation_offset: i64,
}

impl RangeFunction {
    pub fn parse(line: &String) -> RangeFunction {
        let values: Vec<i64> = line.split(' ').map(|field| field.parse::<i64>().unwrap()).collect();

        let destination_start: i64 = values[0];
        let source_start: i64      = values[1];
        let length: i64            = values[2];

        RangeFunction {
            desintation_offset: destination_start - source_start,
            source_range: source_start..(source_start + length)
        }
    }
}

pub type RangeMap = Vec<RangeFunction>;

fn parse_map(lines: &[String]) -> RangeMap {
    lines.iter().map(|line| RangeFunction::parse(line)).collect()
}

pub fn parse_maps(lines: &Vec<String>) -> Vec<RangeMap> {
    lines.split(|line| line.is_empty()).skip(1)
        .map(|block| parse_map(&block[1..])).collect()
}


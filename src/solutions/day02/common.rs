pub struct Round {
    pub red:   usize,
    pub green: usize,
    pub blue:  usize
}

pub struct Game {
    pub id: i64,
    pub rounds: Vec<Round>
}

pub fn parse_round(field: &str) -> Round {
    let subfields: Vec<&str> = field.trim().split(", ").collect();
    let mut round: Round = Round { red: 0, green: 0, blue: 0 };

    for subfield in subfields {
        let mut amount_and_color: std::str::Split<'_, char> = subfield.split(' ');
        let amount: usize = amount_and_color.next().unwrap().parse::<usize>().unwrap();
        let color: &str = amount_and_color.next().unwrap();

        match color {
            "red"   => round.red   = amount,
            "green" => round.green = amount,
            "blue"  => round.blue  = amount,
            _ => panic!("Unknown color '{color}'")
        }
    }

    return round;
}

pub fn parse_game(line: &String) -> Game {
    let mut top_level_fields: std::str::SplitN<'_, char> = line.splitn(2, ':');
    let game_id: i64 = top_level_fields.next().unwrap()[5..].parse::<i64>().unwrap();
    let rounds: Vec<Round> = top_level_fields.next().unwrap()
        .split(';').map(|field| parse_round(field))
        .collect();

    Game { id: game_id, rounds: rounds }
}

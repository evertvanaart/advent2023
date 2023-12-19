/* ------------------------------- Comparator ------------------------------- */

pub enum Comparator {
    GreaterThan(usize),
    LessThan(usize)
}

/* ---------------------------------- Rule ---------------------------------- */

pub enum Rule {
    Check(usize, Comparator, String),
    Always(String)
}

impl Rule {
    pub fn parse(field: &str) -> Rule {
        if let Some(fields) = field.split_once(':') {
            let target: String = String::from(fields.1);
            let (index, cmp) = Self::parse_predicate(fields.0);
            Rule::Check(index, cmp, target)
        } else {
            Rule::Always(String::from(field))
        }
    }

    pub fn parse_predicate(field: &str) -> (usize, Comparator) {
        if let Some(fields) = field.split_once('<') {
            let index: usize = xmas_to_index(fields.0);
            let value: usize = fields.1.parse().unwrap();
            return (index, Comparator::LessThan(value));
        } else if let Some(fields) = field.split_once('>') {
            let index: usize = xmas_to_index(fields.0);
            let value: usize = fields.1.parse().unwrap();
            return (index, Comparator::GreaterThan(value));
        }

        panic!();
    }
}

/* -------------------------------- Workflow -------------------------------- */

pub struct Workflow {
    pub label: String,
    pub rules: Vec<Rule>
}

impl Workflow {
    pub fn parse(line: &str) -> Workflow {
        let (label, rules_str) = line.split_once('{').unwrap();
        let rule_fields: Vec<&str> = rules_str[0 .. rules_str.len() - 1].split(',').collect();
        let rules: Vec<Rule> = rule_fields.into_iter().map(|f| Rule::parse(f)).collect();

        Workflow { label: String::from(label), rules: rules }
    }
}

/* --------------------------------- Helper --------------------------------- */

pub fn xmas_to_index(char_str: &str) -> usize {
    match char_str {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!()
    }
}
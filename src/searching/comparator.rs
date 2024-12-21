use regex::Regex;

pub enum Comparison {
    GreaterThan(i32),
    GreaterThanEqual(i32),
    LessThan(i32),
    LessThanEqual(i32),
    Equal(i32),
    NotEqual(i32)
}

impl Comparison {
    pub fn new(comp: &str, number: i32) -> Option<Comparison> {
        match comp {
            ">"  => Some(Self::GreaterThan(number)),
            ">=" => Some(Self::GreaterThanEqual(number)),
            "<"  => Some(Self::LessThan(number)),
            "<=" => Some(Self::LessThanEqual(number)),
            "="  => Some(Self::Equal(number)),
            "!"  => Some(Self::NotEqual(number)),
            _    => None
        }
    }

    pub fn from_string(string: &str) -> Vec<Comparison> {
        let re = Regex::new(r"([><!=]{1}|>=|<=)(\d+)").unwrap();
        let mut comparisons = vec![];
        for (_, [comp, num]) in re.captures_iter(string).map(|c| c.extract()) {
            if let Ok(n) = num.parse::<i32>() {
                if let Some(comparison) = Comparison::new(comp, n) {
                    comparisons.push(comparison);
                }
            }
        }
        return comparisons;
    }

    pub fn compare(&self, number: i32) -> bool {
        match self {
            Comparison::GreaterThan(n)      => number >  *n,
            Comparison::GreaterThanEqual(n) => number >= *n,
            Comparison::LessThan(n)         => number <  *n,
            Comparison::LessThanEqual(n)    => number <= *n,
            Comparison::Equal(n)            => number == *n,
            Comparison::NotEqual(n)         => number != *n
        }
    }
}

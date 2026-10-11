use std::fmt::{self, Display};
use std::io::{self, BufRead};
use std::str::FromStr;

enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

impl Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Temperature::Celsius(unit) => write!(f, "{}C", unit),
            Temperature::Fahrenheit(unit) => write!(f, "{}F", unit),
        }
    }
}

impl FromStr for Temperature {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty input".to_string());
        }

        let (number, unit) = s.split_at(s.len() - 1);
        let value: i32 = number
            .trim()
            .parse()
            .map_err(|e| format!("Invalid number '{}': {}", number, e))?;

        match unit {
            "C" | "c" => Ok(Temperature::Celsius(value)),
            "F" | "f" => Ok(Temperature::Fahrenheit(value)),
            _ => Err(format!("Unknown unit '{}'. Use C or F.", unit)),
        }
    }
}

fn to_celsius(t: Temperature) -> i32 {
    match t {
        Temperature::Celsius(n) => n,
        Temperature::Fahrenheit(n) => (n - 32) * 5 / 9,
    }
}

fn to_fahrenheit(t: Temperature) -> i32 {
    match t {
        Temperature::Celsius(n) => (n * 9 / 5) + 32,
        Temperature::Fahrenheit(n) => n,
    }
}

fn read_input<R: BufRead>(reader: &mut R) -> Result<String, String> {
    let mut text = String::new();
    reader
        .read_line(&mut text)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    Ok(text.trim().to_string())
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    println!("Please enter the temperature with C or F at the end (e.g. 25C or 77F):");
    let input = match read_input(&mut handle) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let temperature: Temperature = match input.parse() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    match temperature {
        Temperature::Celsius(n) => println!("{}F", to_fahrenheit(Temperature::Celsius(n))),
        Temperature::Fahrenheit(n) => println!("{}C", to_celsius(Temperature::Fahrenheit(n))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("25C")]
    #[case("25c")]
    #[case("25F")]
    #[case("25f")]
    fn test_temperature_parse_ok(#[case] input: &str) {
        assert!(input.parse::<Temperature>().is_ok());
    }

    #[rstest]
    #[case("invalid")]
    #[case("25")]
    #[case("25X")]
    #[case("")]
    fn test_temperature_parse_err(#[case] input: &str) {
        assert!(input.parse::<Temperature>().is_err());
    }

    #[rstest]
    #[case(Temperature::Celsius(0), 0)]
    #[case(Temperature::Celsius(25), 25)]
    #[case(Temperature::Celsius(-40), -40)]
    #[case(Temperature::Celsius(100), 100)]
    fn test_to_celsius_from_celsius(#[case] input: Temperature, #[case] expected: i32) {
        assert_eq!(to_celsius(input), expected);
    }

    #[rstest]
    #[case(Temperature::Fahrenheit(32), 0)]
    #[case(Temperature::Fahrenheit(212), 100)]
    #[case(Temperature::Fahrenheit(-40), -40)]
    #[case(Temperature::Fahrenheit(50), 10)]
    #[case(Temperature::Fahrenheit(68), 20)]
    #[case(Temperature::Fahrenheit(14), -10)]
    fn test_to_celsius_from_fahrenheit(#[case] input: Temperature, #[case] expected: i32) {
        assert_eq!(to_celsius(input), expected);
    }

    #[rstest]
    #[case(Temperature::Fahrenheit(0), 0)]
    #[case(Temperature::Fahrenheit(77), 77)]
    #[case(Temperature::Fahrenheit(-40), -40)]
    #[case(Temperature::Fahrenheit(212), 212)]
    fn test_to_fahrenheit_from_fahrenheit(#[case] input: Temperature, #[case] expected: i32) {
        assert_eq!(to_fahrenheit(input), expected);
    }

    #[rstest]
    #[case(Temperature::Celsius(0), 32)]
    #[case(Temperature::Celsius(100), 212)]
    #[case(Temperature::Celsius(-40), -40)]
    #[case(Temperature::Celsius(10), 50)]
    #[case(Temperature::Celsius(20), 68)]
    #[case(Temperature::Celsius(-10), 14)]
    fn test_to_fahrenheit_from_celsius(#[case] input: Temperature, #[case] expected: i32) {
        assert_eq!(to_fahrenheit(input), expected);
    }
}

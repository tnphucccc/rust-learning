use std::io;

#[derive(Debug, Clone, Copy)]
enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl Unit {
    fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'C' => Some(Self::Celsius),
            'F' => Some(Self::Fahrenheit),
            'K' => Some(Self::Kelvin),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Celsius => "C",
            Self::Fahrenheit => "F",
            Self::Kelvin => "K",
        }
    }
}

// Convert any temperature to Celsius.
fn to_celsius(value: f64, unit: Unit) -> f64 {
    match unit {
        Unit::Celsius => value,
        Unit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
        Unit::Kelvin => value - 273.15,
    }
}

// Convert Celsius to the requested unit.
fn from_celsius(celsius: f64, unit: Unit) -> f64 {
    match unit {
        Unit::Celsius => celsius,
        Unit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        Unit::Kelvin => celsius + 273.15,
    }
}

fn parse_input(input: &str) -> Result<(f64, Unit), &'static str> {
    let trimmed = input.trim();

    // Find the last character safely (UTF-8 aware).
    let (unit_index, unit_char) = trimmed
        .char_indices()
        .next_back()
        .ok_or("Input cannot be empty.")?;

    let unit = Unit::from_char(unit_char).ok_or("Unknown unit. Please use C, F, or K.")?;

    let number = trimmed[..unit_index].trim();

    let temperature: f64 = number.parse().map_err(|_| "Invalid number format.")?;

    if !temperature.is_finite() {
        return Err("Temperature must be a finite number.");
    }

    // Validate absolute zero.
    if to_celsius(temperature, unit) < -273.15 {
        return Err("Temperature cannot be below absolute zero.");
    }

    Ok((temperature, unit))
}

fn main() {
    println!("Temperature Converter!");
    println!("Supported units: Celsius (C), Fahrenheit (F), Kelvin (K)");
    println!("Examples: 32F, 32 F, 50C, 300K");
    println!("Type 'quit' or 'exit' to exit.");

    loop {
        println!("\nPlease input your temperature:");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nEnd of input. Goodbye!");
                break;
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("Failed to read input: {error}");
                break;
            }
        }

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("quit") || trimmed.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        let (temperature, source_unit) = match parse_input(trimmed) {
            Ok(result) => result,
            Err(error) => {
                println!("Error: {error}");
                continue;
            }
        };

        // Convert through Celsius.
        let celsius = to_celsius(temperature, source_unit);

        // Select a target unit.
        // For now, convert to all other units.
        println!("{temperature} {} is:", source_unit.as_str());

        for target_unit in [Unit::Celsius, Unit::Fahrenheit, Unit::Kelvin] {
            if target_unit as u8 == source_unit as u8 {
                continue;
            }

            let result = from_celsius(celsius, target_unit);

            println!("{result:.2} {}", target_unit.as_str());
        }
    }
}

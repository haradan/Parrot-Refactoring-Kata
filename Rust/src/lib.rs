trait Parrot {
    fn speed(&self) -> Result<f32, &'static str>;
}

struct NorwegianBlue {
    voltage: f32,
    nailed: bool,
}

impl Parrot for NorwegianBlue {
    fn speed(&self) -> Result<f32, &'static str> {
        if self.nailed {
            Ok(0.0)
        } else {
            Ok(compute_base_speed_for_voltage(self.voltage))
        }
    }
}

struct EuropeanParrot {}

impl Parrot for EuropeanParrot {
    fn speed(&self) -> Result<f32, &'static str> {
        Ok(base_speed())
    }
}

struct AfricanParrot {
    number_of_coconuts: usize,
}

impl Parrot for AfricanParrot {
    fn speed(&self) -> Result<f32, &'static str> {
        Ok(f32::max(
            0.0,
            base_speed() - load_factor() * self.number_of_coconuts as f32,
        ))
    }
}

fn compute_base_speed_for_voltage(voltage: f32) -> f32 {
    let fixed_base_speed = 24.0;
    let base_speed_for_voltage = voltage * base_speed();
    if base_speed_for_voltage < fixed_base_speed {
        base_speed_for_voltage
    } else {
        fixed_base_speed
    }
}

fn load_factor() -> f32 {
    9.0
}

fn base_speed() -> f32 {
    12.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use itertools::Itertools;

    #[test]
    fn parrot_speeds() {
        let parrots: &[(&str, Box<dyn Parrot>)] = &[
            ("European parrot", Box::new(EuropeanParrot {})),
            (
                "Unladen african parrot",
                Box::new(AfricanParrot {
                    number_of_coconuts: 0,
                }),
            ),
            (
                "African parrot with 1 coconut",
                Box::new(AfricanParrot {
                    number_of_coconuts: 1,
                }),
            ),
            (
                "African parrot with 2 coconuts",
                Box::new(AfricanParrot {
                    number_of_coconuts: 2,
                }),
            ),
            (
                "Norwegian blue",
                Box::new(NorwegianBlue {
                    voltage: 0.0,
                    nailed: false,
                }),
            ),
            (
                "1.5V Norwegian blue",
                Box::new(NorwegianBlue {
                    voltage: 1.5,
                    nailed: false,
                }),
            ),
            (
                "4V Norwegian blue",
                Box::new(NorwegianBlue {
                    voltage: 4.0,
                    nailed: false,
                }),
            ),
            (
                "4V nailed Norwegian blue",
                Box::new(NorwegianBlue {
                    voltage: 4.0,
                    nailed: true,
                }),
            ),
        ];
        let printed: String = parrots
            .iter()
            .map(|tuple| format!("{:>31}: {:>2}", tuple.0, tuple.1.speed().unwrap()))
            .join("\n");
        assert_snapshot!(printed);
    }
}

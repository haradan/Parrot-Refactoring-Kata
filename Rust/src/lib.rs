pub trait Parrot {
    fn speed(&self) -> Result<f32, &'static str>;
}

pub fn european_parrot() -> Box<dyn Parrot> {
    Box::new(EuropeanParrot {})
}

pub fn unladen_african_parrot() -> Box<dyn Parrot> {
    Box::new(AfricanParrot {
        number_of_coconuts: 0,
    })
}

pub fn african_parrot_with_coconuts(coconuts: usize) -> Box<dyn Parrot> {
    Box::new(AfricanParrot {
        number_of_coconuts: coconuts,
    })
}

pub fn norwegian_blue_with_volts(voltage: f32) -> Box<dyn Parrot> {
    Box::new(NorwegianBlue {
        voltage,
        nailed: false,
    })
}

pub fn nailed_norwegian_blue_with_volts(voltage: f32) -> Box<dyn Parrot> {
    Box::new(NorwegianBlue {
        voltage,
        nailed: true,
    })
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

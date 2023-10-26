use insta::assert_snapshot;
use itertools::Itertools;
use pollymorphism::{AfricanParrot, EuropeanParrot, NorwegianBlue, Parrot};

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
            "1V Norwegian blue",
            Box::new(NorwegianBlue {
                voltage: 1.0,
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
    let parrot_speeds: String = parrots
        .iter()
        .map(|tuple| format!("{:>31}: {:>2}", tuple.0, tuple.1.speed().unwrap()))
        .join("\n");
    assert_snapshot!(parrot_speeds);
}

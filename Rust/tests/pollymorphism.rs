use insta::assert_snapshot;
use itertools::Itertools;
use pollymorphism::{
    african_parrot_with_coconuts, european_parrot, nailed_norwegian_blue_with_volts,
    norwegian_blue_with_volts, unladen_african_parrot,
};

#[test]
fn parrot_speeds() {
    let parrots = &[
        ("European parrot", european_parrot()),
        ("Unladen african parrot", unladen_african_parrot()),
        (
            "African parrot with 1 coconut",
            african_parrot_with_coconuts(1),
        ),
        (
            "African parrot with 2 coconuts",
            african_parrot_with_coconuts(2),
        ),
        ("Norwegian blue", norwegian_blue_with_volts(0.0)),
        ("1V Norwegian blue", norwegian_blue_with_volts(1.0)),
        ("1.5V Norwegian blue", norwegian_blue_with_volts(1.5)),
        ("4V Norwegian blue", norwegian_blue_with_volts(4.0)),
        (
            "4V nailed Norwegian blue",
            nailed_norwegian_blue_with_volts(4.0),
        ),
    ];
    let parrot_speeds: String = parrots
        .iter()
        .map(|tuple| format!("{:>31}: {:>2}", tuple.0, tuple.1.speed().unwrap()))
        .join("\n");
    assert_snapshot!(parrot_speeds);
}

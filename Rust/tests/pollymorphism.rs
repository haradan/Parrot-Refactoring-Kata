use insta::assert_snapshot;
use pollymorphism::*;

fn case<P: Parrot>(description: &str, parrot: P) -> String {
    format!("{:>31}: {:>2}", description, parrot.speed().unwrap())
}

#[test]
fn parrot_speeds() {
    let cases = &[
        case("European parrot", european_parrot()),
        case("Unladen african parrot", unladen_african_parrot()),
        case(
            "African parrot with 1 coconut",
            african_parrot_with_coconuts(1),
        ),
        case(
            "African parrot with 2 coconuts",
            african_parrot_with_coconuts(2),
        ),
        case("Norwegian blue", norwegian_blue_with_volts(0.0)),
        case("1V Norwegian blue", norwegian_blue_with_volts(1.0)),
        case("1.5V Norwegian blue", norwegian_blue_with_volts(1.5)),
        case("4V Norwegian blue", norwegian_blue_with_volts(4.0)),
        case(
            "4V nailed Norwegian blue",
            nailed_norwegian_blue_with_volts(4.0),
        ),
    ];
    let parrot_speeds = cases.join("\n");
    assert_snapshot!(parrot_speeds);
}

fn main() {
    let x: f64 = 5.33;
    print_labeled_measurement(x, "ft");
    println!("Converting your input to inches.");
    print_labeled_measurement(convert_ft_to_in(x), "in");
}

fn print_labeled_measurement (value: f64, unit_label: &str) {
    println!("Your input is {value} {unit_label}");
}

fn convert_ft_to_in (feet: f64) -> f64 {
    let conversion = feet * tweleve();
    return conversion
}

fn tweleve () -> f64{
    12.0
}
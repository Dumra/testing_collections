mod services;

fn main() {
    let mut vector = vec![1, 5, -8, -25, 64];
    println!("Input Vector: {:?}", vector);
    println!("Mean is: {}", services::vectors::calculate_mean(&vector));

    println!(
        "Median is: {}",
        services::vectors::calculate_median(&mut vector)
    );
    let mut vector = vec![1, 5, -8, 35, -25, 64];
    println!(
        "Median is: {}",
        services::vectors::calculate_median(&mut vector)
    );

    let vector = vec![1, 5, 35, 32, 1, 3, 8, 4, 1, 32, -8, 35, -25, 64];
    println!("Vector: {:?}", vector);
    let mode = services::vectors::calculate_mode(&vector);
    println!("The mode is {}, with counts: {} ", mode.0, mode.1);

    println!("Please input your string!");
    println!("Output: {}", services::strings::read_input());

    println!("Hash map exercise");
    services::hash_map::run();
}

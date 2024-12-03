pub fn run<T>(input: String, part_1: fn(input: String) -> T, part_2: fn(input: String) -> T)
where
    T: std::fmt::Display,
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "part_1" => {
                let result = part_1(input);
                println!("part 1 solution: {}", result);
            }
            "part_2" => {
                let result = part_2(input);
                println!("part 2 solution: {}", result);
            }
            _ => {
                println!("Usage: <day> <part>");
                std::process::exit(64);
            }
        }
    } else {
        println!("Usage: part_<1 | 2>");
        std::process::exit(64);
    }
}

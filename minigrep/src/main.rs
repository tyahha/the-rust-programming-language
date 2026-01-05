fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("In file {}", filename);

    let f = std::fs::File::open(filename).expect("File not found");

    let contents = std::io::read_to_string(f).expect("Something went wrong reading the file");

    println!("With query {}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    (&args[1], &args[2])
}

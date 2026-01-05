fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = parse_config(&args);

    println!("In file {}", config.filename);

    let f = std::fs::File::open(config.filename).expect("File not found");

    let contents = std::io::read_to_string(f).expect("Something went wrong reading the file");

    println!("With query {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}

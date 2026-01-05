fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args);

    println!("In file {}", config.filename);

    let f = std::fs::File::open(config.filename).expect("File not found");

    let contents = std::io::read_to_string(f).expect("Something went wrong reading the file");

    println!("With query {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

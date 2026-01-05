pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("In file {}", config.filename);

    let f = std::fs::File::open(config.filename)?;

    let contents = std::io::read_to_string(f)?;

    println!("With query {}", contents);

    Ok(())
}
use clap::{Arg, Command};

struct Shortcut {
    keys: String,
    description: String,
}

impl Shortcut {
    fn new(keys: String, description: String) -> Shortcut {
        Shortcut { keys, description }
    }
}

struct Shortcuts {
    keys_vector: Vec<Shortcut>,
}

fn main() {
    let matches = Command::new("nvsc")
        .about("Neovim shortcuts")
        .author("Maskedball <maskedballmail@gmail.com>")
        .arg(
            Arg::new("QUERY")
                .long("query")
                .short('q')
                .help("Search shortcuts by description")
                .required(false),
        )
        .get_matches();

    let query = matches.get_one::<String>("QUERY");

    if query.is_some() && query.unwrap().is_empty() == false {
        println!("Query is: {}", query.unwrap());

        std::process::exit(0)
    }

    println!("Run without query...");
}

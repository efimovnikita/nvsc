use clap::{Arg, Command};
use prettytable::{color, format, Attr, Cell, Row, Table};

struct Shortcut {
    keys: String,
    description: String,
}

impl Shortcut {
    fn new(keys: String, description: String) -> Shortcut {
        Shortcut { keys, description }
    }

    fn get_row(&self) -> Row {
        Row::new(vec![
            Cell::new(self.keys.as_str()).with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new(self.description.as_str()).with_style(Attr::Italic(true)),
        ])
    }
}

struct ShortcutsCategory {
    keys_vector: Vec<Shortcut>,
    name: String,
}

impl ShortcutsCategory {
    fn new(name: String, keys_vector: Vec<Shortcut>) -> ShortcutsCategory {
        ShortcutsCategory { name, keys_vector }
    }

    fn get_category_table(&self) -> Table {
        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        table.set_titles(Row::new(vec![Cell::new(&self.name.as_str()).with_hspan(2)]));

        for k in &self.keys_vector {
            table.add_row(k.get_row());
        }

        table
    }
}

struct Categories {
    categories: Vec<ShortcutsCategory>,
}

impl Categories {
    fn new() -> Categories {
        Categories {
            categories: vec![
                ShortcutsCategory::new(
                    "Insertions".to_string(),
                    vec![
                        Shortcut::new("di(".to_string(), "Delete inside parenthesis".to_string()),
                        Shortcut::new(
                            "ci(".to_string(),
                            "Delete and insert inside parenthesis".to_string(),
                        ),
                    ],
                ),
                ShortcutsCategory::new(
                    "Movements".to_string(),
                    vec![Shortcut::new(
                        "zz".to_string(),
                        "Center this line".to_string(),
                    )],
                ),
            ],
        }
    }

    fn print_all(&self) {
        for c in &self.categories {
            c.get_category_table().printstd();
            println!();
        }
    }
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

    Categories::new().print_all();
}

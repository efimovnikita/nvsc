use clap::{Arg, Command};
use prettytable::{color, format, Attr, Cell, Row, Table};

struct Shortcut {
    keys: String,
    description: String,
}

impl Shortcut {
    fn new(keys: &str, description: &str) -> Shortcut {
        Shortcut {
            keys: keys.to_string(),
            description: description.to_string(),
        }
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
    fn new(name: &str, keys_vector: Vec<Shortcut>) -> ShortcutsCategory {
        ShortcutsCategory {
            name: name.to_string(),
            keys_vector,
        }
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

    fn filter_out_by_query(&mut self, query: &str) {
        let query = query.to_lowercase();
        self.keys_vector
            .retain(|s| s.description.to_lowercase().contains(&query));
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
                    "Editing",
                    vec![
                        Shortcut::new("di(", "Delete content between parenthesis"),
                        Shortcut::new(
                            "ci(",
                            "Delete old and insert new content between parenthesis",
                        ),
                        Shortcut::new("cw", "Delete word and active insert mode (change word)"),
                        Shortcut::new("dw", "Delete word"),
                        Shortcut::new("caw", "Delete whole word and activate insert mode (change word, cursor position doesn't matter)"),
                        Shortcut::new("daw", "Delete whole word (cursor position doesn't matter)"),
                        Shortcut::new("ct)", "Delete content til symbol ')' and active insert mode"),
                        Shortcut::new("dt)", "Delete content til symbol ')'"),
                        Shortcut::new("a", "Append after cursor"),
                        Shortcut::new("A", "Append from end of the current line"),
                        Shortcut::new("i", "Append before cursor"),
                        Shortcut::new("o", "Insert next line after current line"),
                        Shortcut::new("O", "Insert next line before current line"),
                        Shortcut::new("S", "Delete whole line and insert"),
                        Shortcut::new("C", "Delete content until the end of the line and insert"),
                        Shortcut::new("cl", "Delete current letter and insert (change letter)")
                    ],
                ),
                ShortcutsCategory::new(
                    "Cursor movement",
                    vec![
                        Shortcut::new("zz", "Center cursor on screen"),
                        Shortcut::new("b", "Jump backwards to the start of a word"),
                        Shortcut::new("w", "Jump forwards to the start of a word"),
                        Shortcut::new("e", "Jump forwards to the end of a word"),
                        Shortcut::new("Ctlr + u", "Move back 1/2 a screen"),
                        Shortcut::new("Ctlr + d", "Move forward 1/2 a screen"),
                        Shortcut::new("0", "Start of the line"),
                        Shortcut::new("^", "Start of the line after whitespaces and tabs"),
                        Shortcut::new("$", "End of the line"),
                        Shortcut::new("gg", "First line of the document"),
                        Shortcut::new("G", "Last line of the document"),
                        Shortcut::new("f(", "Find symbol '(' forward"),
                        Shortcut::new("F(", "Find symbol '(' backward"),
                    ],
                ),
                ShortcutsCategory::new("Exiting", 
                    vec![
                        Shortcut::new("ZZ", "Save and quit"),Shortcut::new("ZQ", "Quit without changes"),
                    ]
                ),
            ],
        }
    }

    fn print_all(&self) {
        println!();
        for c in &self.categories {
            c.get_category_table().printstd();
            println!();
        }
    }

    fn print_by_query(&mut self, query: &str) {
        println!();
        for c in &mut self.categories {
            c.filter_out_by_query(&query);
            if c.keys_vector.is_empty() == false {
                c.get_category_table().printstd();
                println!();
            }
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
        Categories::new().print_by_query(query.unwrap());
        std::process::exit(0)
    }

    Categories::new().print_all();
}

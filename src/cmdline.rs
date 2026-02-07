use super::view_model::AppViewModel;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Language {
    #[value(name = "en")]
    English,
    #[value(name = "fr")]
    French,
    #[value(name = "de")]
    German,
    #[value(name = "it")]
    Italian,
    #[value(name = "ru")]
    Russian,
    #[value(name = "es")]
    Spanish,
}

impl Language {
    fn as_str(&self) -> &str {
        match self {
            Language::English => "en",
            Language::French => "fr",
            Language::German => "de",
            Language::Italian => "it",
            Language::Russian => "ru",
            Language::Spanish => "es",
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "number2text")]
#[command(author = "Konstantin Adamov <xrayadamo@gmail.com>")]
#[command(version)]
#[command(about = "A number to text converter application", long_about = None)]
pub struct Cli {
    /// Specify language for conversion
    #[arg(short = 'l', long = "language", value_enum)]
    pub language: Option<Language>,

    /// Number to convert to text
    #[arg(value_name = "NUMBER")]
    pub number: Option<i64>,
}

pub fn parse_cmdline_args() -> bool {
    let cli = Cli::parse();

    match (cli.language, cli.number) {
        (Some(language), Some(number)) => {
            let view_model = AppViewModel::new();
            match view_model.convert_by_language(language.as_str(), number) {
                Some(result) => println!("{}", result),
                None => println!("Could not convert number."),
            }
            true
        }
        (Some(_), None) => {
            eprintln!("Error: Please specify a number to convert after the language code.");
            std::process::exit(1);
        }
        (None, Some(_)) => {
            eprintln!("Error: Please specify a language using -l or --language.");
            std::process::exit(1);
        }
        (None, None) => {
            // No arguments, launch GUI
            false
        }
    }
}

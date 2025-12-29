use super::view_model::AppViewModel;

pub fn parse_cmdline_args(args: &[String])  {
    if args[0] == "--version" || args[1] == "-v" {
        print_version();
        return;
    }

    if args[0] == "--help" || args[1] == "-h" {
        print_help();
        return;
    }
    let languages = vec!["en", "fr", "de", "it", "ru", "es"];
    let language ;
    if args[0] == "--language" || args[1] == "-l" {
        if languages.contains(&args[2].as_str()) == false {
            println!("Please specify a language code after --language or -l.");
            return;
        }

        language = args[2].as_str();
        
        if args.get(3).is_none() {
            println!("Please specify a number to convert after the language code.");
            return;
        }

        if let Ok(number) = args[3].parse::<i64>(){
            let view_model = AppViewModel::new();    
            match view_model.convert_by_language(language, number) {
                Some(result) => println!("{}", result),
                None => println!("Could not convert number."),
            }
            return;
        } else {
            println!("Please specify a valid number to convert after the language code.");
        }
        
        return;
    }

    println!("Unknown command line argument. Use --help to see available options.");
    
}

fn print_version(){
    println!("Number 2 Text version {}", AppViewModel::get_app_version());
}

fn print_help(){
    println!("Number 2 Text - A number to text converter application.");
    println!();
    println!("Usage: number2text [OPTIONS] number");
    println!();
    println!("Options:");
    println!("  -v, --version       Show application version");
    println!("  -h, --help          Show this help message");
    println!("  -l, --language      Specify language for conversion (Available languages: en, fr, de, it, ru, es)", );
}

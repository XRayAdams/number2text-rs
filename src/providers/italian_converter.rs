/*
    Copyright 2025 Konstantin Adamov
    Licenced under MIT Licence (https://opensource.org/licenses/MIT)
*/

use super::base_converter::BaseConverter;

pub struct ItalianConverter;

impl ItalianConverter {
    const ONES: [&'static str; 20] = [
        "zero", "uno", "due", "tre", "quattro", "cinque", "sei", "sette", "otto", "nove",
        "dieci", "undici", "dodici", "tredici", "quattordici", "quindici", "sedici",
        "diciassette", "diciotto", "diciannove",
    ];

    const TENS: [&'static str; 10] = [
        "", "", "venti", "trenta", "quaranta", "cinquanta", "sessanta", "settanta", "ottanta", "novanta",
    ];
}

impl BaseConverter for ItalianConverter {
    fn name(&self) -> &str {
        "Italian"
    }
    
    fn short_name(&self) -> &str {
        "it"
    }

    fn native_number_too_large_error_text(&self) -> &str {
        "Numero troppo grande"
    }

    fn convert(&self, input: i64) -> String {
        if input > 999_999_999_999 {
            return self.native_number_too_large_error_text().to_string();
        }

        if input < 0 {
            return format!("meno {}", self.convert(-input));
        }

        if input < 20 {
            return Self::ONES[input as usize].to_string();
        }

        if input < 100 {
            let ten = input / 10;
            let unit = input % 10;

            if unit == 0 {
                return Self::TENS[ten as usize].to_string();
            }

            let tens_word = Self::TENS[ten as usize];
            if unit == 1 || unit == 8 {
                return format!("{}{}", &tens_word[..tens_word.len() - 1], Self::ONES[unit as usize]);
            }

            return format!("{}{}", tens_word, Self::ONES[unit as usize]);
        }

        if input < 1000 {
            let hundred = input / 100;
            let remainder = input % 100;

            let hundreds_str = if hundred == 1 {
                "cento".to_string()
            } else {
                format!("{}cento", Self::ONES[hundred as usize])
            };

            if remainder == 0 {
                return hundreds_str;
            }

            if remainder % 10 == 1 || remainder % 10 == 8 {
                return format!("{}{}", &hundreds_str[..hundreds_str.len() - 1], self.convert(remainder));
            }

            return format!("{}{}", hundreds_str, self.convert(remainder));
        }

        if input < 1_000_000 {
            let thousands = input / 1000;
            let remainder = input % 1000;

            let thousands_str = if thousands == 1 {
                "mille".to_string()
            } else {
                format!("{}mila", self.convert(thousands))
            };

            return if remainder == 0 {
                thousands_str
            } else {
                format!("{}{}", thousands_str, self.convert(remainder))
            };
        }

        if input < 1_000_000_000 {
            if input == 1_000_000 {
                return "un milione".to_string();
            }

            let millions = input / 1_000_000;
            let remainder = input % 1_000_000;

            return if remainder != 0 {
                format!("{} milioni {}", self.convert(millions), self.convert(remainder))
            } else {
                format!("{} milioni", self.convert(millions))
            };
        }

        if input < 1_000_000_000_000 {
            if input == 1_000_000_000 {
                return "un miliardo".to_string();
            }

            let billions = input / 1_000_000_000;
            let remainder = input % 1_000_000_000;

            return if remainder != 0 {
                format!("{} miliardi {}", self.convert(billions), self.convert(remainder))
            } else {
                format!("{} miliardi", self.convert(billions))
            };
        }

        self.native_number_too_large_error_text().to_string()
    }
}

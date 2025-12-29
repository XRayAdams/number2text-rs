/*
    Copyright 2025 Konstantin Adamov
    Licenced under MIT Licence (https://opensource.org/licenses/MIT)
*/


use super::base_converter::BaseConverter;

pub struct GermanConverter;

impl GermanConverter {
    const ONES: [&'static str; 20] = [
        "null", "eins", "zwei", "drei", "vier", "fünf", "sechs", "sieben", "acht", "neun",
        "zehn", "elf", "zwölf", "dreizehn", "vierzehn", "fünfzehn", "sechzehn",
        "siebzehn", "achtzehn", "neunzehn",
    ];

    const TENS: [&'static str; 10] = [
        "", "", "zwanzig", "dreißig", "vierzig", "fünfzig", "sechzig", "siebzig", "achtzig", "neunzig",
    ];
}

impl BaseConverter for GermanConverter {
    fn name(&self) -> &str {
        "German"
    }

    fn native_number_too_large_error_text(&self) -> &str {
        "Nummer zu groß"
    }

    fn short_name(&self) -> &str {
        "de"
    }

    fn convert(&self, input: i64) -> String {
        if input > 999_999_999_999 {
            return self.native_number_too_large_error_text().to_string();
        }

        if input < 0 {
            return format!("minus {}", self.convert(-input));
        }

        if input == 0 {
            return "null".to_string();
        }

        if input < 20 {
            return Self::ONES[input as usize].to_string();
        }

        if input < 100 {
            let ten = input / 10;
            let unit = input % 10;

            if unit == 0 {
                return Self::TENS[ten as usize].to_string();
            } else if unit == 1 {
                return format!("einund{}", Self::TENS[ten as usize]);
            }

            return format!("{}und{}", Self::ONES[unit as usize], Self::TENS[ten as usize]);
        }

        if input < 1000 {
            let hundred = input / 100;
            let remainder = input % 100;

            let hundreds_str = if hundred > 1 {
                format!("{}hundert", Self::ONES[hundred as usize])
            } else {
                "hundert".to_string()
            };

            return if remainder == 0 {
                hundreds_str
            } else {
                format!("{}{}", hundreds_str, self.convert(remainder))
            };
        }

        if input < 1_000_000 {
            let thousands = input / 1000;
            let remainder = input % 1000;

            let thousands_str = if thousands == 1 {
                "eintausend".to_string()
            } else {
                format!("{}tausend", self.convert(thousands))
            };

            return if remainder == 0 {
                thousands_str
            } else {
                format!("{}{}", thousands_str, self.convert(remainder))
            };
        }

        if input < 1_000_000_000 {
            let millions = input / 1_000_000;
            let remainder = input % 1_000_000;

            let millions_str = if millions == 1 {
                "eine Million".to_string()
            } else {
                format!("{} Millionen", self.convert(millions))
            };

            return if remainder == 0 {
                millions_str
            } else {
                format!("{} {}", millions_str, self.convert(remainder))
            };
        }

        if input < 1_000_000_000_000 {
            let billions = input / 1_000_000_000;
            let remainder = input % 1_000_000_000;

            let billions_str = if billions == 1 {
                "eine Milliarde".to_string()
            } else {
                format!("{} Milliarden", self.convert(billions))
            };

            return if remainder == 0 {
                billions_str
            } else {
                format!("{} {}", billions_str, self.convert(remainder))
            };
        }

        self.native_number_too_large_error_text().to_string()
    }
}

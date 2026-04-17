/*
    Copyright 2025 Konstantin Adamov
    Licenced under MIT Licence (https://opensource.org/licenses/MIT)
*/

use super::base_converter::BaseConverter;

pub struct IndonesianConverter;

impl IndonesianConverter {
    const ONES: [&'static str; 10] = [
        "nol", "satu", "dua", "tiga", "empat", "lima", "enam", "tujuh", "delapan", "sembilan",
    ];
}

// Implementation details for Indonesian number conversion (Bahasa Indonesia)
impl BaseConverter for IndonesianConverter {
    fn name(&self) -> &str {
        "Indonesian"
    }

    fn short_name(&self) -> &str {
        "id"
    }

    fn native_number_too_large_error_text(&self) -> &str {
        "Angka terlalu besar"
    }

    fn convert(&self, input: i64) -> String {
        if input > 999_999_999_999 {
            return self.native_number_too_large_error_text().to_string();
        }

        if input < 0 {
            return format!("minus {}", self.convert(-input));
        }

        if input == 0 {
            return Self::ONES[0].to_string();
        }

        if input < 10 {
            return Self::ONES[input as usize].to_string();
        }

        if input < 20 {
            let ones_digit = input % 10;
            return if ones_digit == 1 {
                "sebelas".to_string()
            } else {
                format!("{} belas", Self::ONES[ones_digit as usize])
            };
        }

        if input < 100 {
            let tens_digit = input / 10;
            let ones_digit = input % 10;
            return if ones_digit != 0 {
                format!("{} puluh {}", Self::ONES[tens_digit as usize], Self::ONES[ones_digit as usize])
            } else {
                format!("{} puluh", Self::ONES[tens_digit as usize])
            };
        }

        if input < 1000 {
            let hundreds = input / 100;
            let remainder = input % 100;
            let hundreds_str = if hundreds == 1 {
                "seratus".to_string()
            } else {
                format!("{} ratus", Self::ONES[hundreds as usize])
            };
            return if remainder != 0 {
                format!("{} {}", hundreds_str, self.convert(remainder))
            } else {
                hundreds_str
            };
        }

        if input < 1_000_000 {
            let thousands = input / 1000;
            let remainder = input % 1000;
            let thousands_str = if thousands == 1 {
                "seribu".to_string()
            } else {
                format!("{} ribu", self.convert(thousands))
            };
            return if remainder != 0 {
                format!("{} {}", thousands_str, self.convert(remainder))
            } else {
                thousands_str
            };
        }

        if input < 1_000_000_000 {
            let millions = input / 1_000_000;
            let remainder = input % 1_000_000;
            return if remainder != 0 {
                format!("{} juta {}", self.convert(millions), self.convert(remainder))
            } else {
                format!("{} juta", self.convert(millions))
            };
        }

        if input < 1_000_000_000_000 {
            let billions = input / 1_000_000_000;
            let remainder = input % 1_000_000_000;
            return if remainder != 0 {
                format!("{} miliar {}", self.convert(billions), self.convert(remainder))
            } else {
                format!("{} miliar", self.convert(billions))
            };
        }

        self.native_number_too_large_error_text().to_string()
    }
}

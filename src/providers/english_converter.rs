use super::base_converter::BaseConverter;

pub struct EnglishConverter;

impl EnglishConverter {
    const ONES: [&'static str; 20] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen",
    ];

    const TENS: [&'static str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
}

impl BaseConverter for EnglishConverter {
    fn name(&self) -> &str {
        "English"
    }

    fn native_number_too_large_error_text(&self) -> &str {
        "Number too large"
    }

    fn convert(&self, input: i64) -> String {
        if input > 999_999_999_999 {
            return self.native_number_too_large_error_text().to_string();
        }

        if input < 0 {
            return format!("minus {}", self.convert(-input));
        }

        if input < 20 {
            return Self::ONES[input as usize].to_string();
        }

        if input < 100 {
            let tens_digit = input / 10;
            let ones_digit = input % 10;
            return if ones_digit != 0 {
                format!("{} {}", Self::TENS[tens_digit as usize], Self::ONES[ones_digit as usize])
            } else {
                Self::TENS[tens_digit as usize].to_string()
            };
        }

        if input < 1000 {
            let hundreds = input / 100;
            let remainder = input % 100;
            return if remainder != 0 {
                format!("{} hundred and {}", Self::ONES[hundreds as usize], self.convert(remainder))
            } else {
                format!("{} hundred", Self::ONES[hundreds as usize])
            };
        }

        if input < 1_000_000 {
            let thousands = input / 1000;
            let remainder = input % 1000;
            return if remainder != 0 {
                format!("{} thousand {}", self.convert(thousands), self.convert(remainder))
            } else {
                format!("{} thousand", self.convert(thousands))
            };
        }

        if input < 1_000_000_000 {
            let millions = input / 1_000_000;
            let remainder = input % 1_000_000;
            return if remainder != 0 {
                format!("{} million {}", self.convert(millions), self.convert(remainder))
            } else {
                format!("{} million", self.convert(millions))
            };
        }

        if input < 1_000_000_000_000 {
            let billions = input / 1_000_000_000;
            let remainder = input % 1_000_000_000;
            return if remainder != 0 {
                format!("{} billion {}", self.convert(billions), self.convert(remainder))
            } else {
                format!("{} billion", self.convert(billions))
            };
        }

        self.native_number_too_large_error_text().to_string()
    }
}

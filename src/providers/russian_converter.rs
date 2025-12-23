use super::base_converter::BaseConverter;

pub struct RussianConverter;

impl RussianConverter {
    const ONES: [&'static str; 20] = [
        "ноль", "один", "два", "три", "четыре", "пять", "шесть", "семь", "восемь", "девять",
        "десять", "одиннадцать", "двенадцать", "тринадцать", "четырнадцать", "пятнадцать",
        "шестнадцать", "семнадцать", "восемнадцать", "девятнадцать",
    ];

    const TENS: [&'static str; 10] = [
        "", "", "двадцать", "тридцать", "сорок", "пятьдесят", "шестьдесят", "семьдесят",
        "восемьдесят", "девяносто",
    ];

    const HUNDREDS: [&'static str; 10] = [
        "", "сто", "двести", "триста", "четыреста", "пятьсот", "шестьсот", "семьсот",
        "восемьсот", "девятьсот",
    ];

    fn convert_internal(&self, input: i64, feminine: bool) -> String {
        if input == 0 {
            return String::new();
        }

        if input < 1000 {
            return self.convert_less_than_thousand(input, feminine);
        }

        let denominations: [(i64, &str, &str, &str, bool); 3] = [
            (1_000_000_000, "миллиард", "миллиарда", "миллиардов", false),
            (1_000_000, "миллион", "миллиона", "миллионов", false),
            (1000, "тысяча", "тысячи", "тысяч", true),
        ];

        for (limit, one, two, five, fem) in &denominations {
            if input >= *limit {
                let head = input / limit;
                let tail = input % limit;
                let head_str = self.convert_internal(head, *fem);
                let plural_str = Self::pluralize(head, one, two, five);
                let tail_str = if tail > 0 {
                    format!(" {}", self.convert_internal(tail, false))
                } else {
                    String::new()
                };
                return format!("{} {}{}", head_str, plural_str, tail_str).trim().to_string();
            }
        }

        String::new()
    }

    fn convert_less_than_thousand(&self, mut num: i64, feminine: bool) -> String {
        if num == 0 {
            return String::new();
        }

        let mut parts = Vec::new();

        let hundreds = num / 100;
        if hundreds > 0 {
            parts.push(Self::HUNDREDS[hundreds as usize].to_string());
            num %= 100;
        }

        if num >= 20 {
            let tens = num / 10;
            parts.push(Self::TENS[tens as usize].to_string());
            num %= 10;
        }

        if num > 0 {
            if feminine && (num == 1 || num == 2) {
                parts.push(if num == 1 { "одна" } else { "две" }.to_string());
            } else {
                parts.push(Self::ONES[num as usize].to_string());
            }
        }

        parts.join(" ")
    }

    fn pluralize(count: i64, one: &str, two: &str, five: &str) -> String {
        let last_digit = count % 10;
        let last_two_digits = count % 100;

        if (11..=19).contains(&last_two_digits) {
            return five.to_string();
        }
        if last_digit == 1 {
            return one.to_string();
        }
        if (2..=4).contains(&last_digit) {
            return two.to_string();
        }
        five.to_string()
    }
}

impl BaseConverter for RussianConverter {
    fn name(&self) -> &str {
        "Russian"
    }

    fn native_number_too_large_error_text(&self) -> &str {
        "число слишком большое"
    }

    fn convert(&self, input: i64) -> String {
        if input > 999_999_999_999 {
            return self.native_number_too_large_error_text().to_string();
        }

        if input < 0 {
            return format!("минус {}", self.convert_internal(input.abs(), false));
        }

        if input == 0 {
            return "ноль".to_string();
        }

        self.convert_internal(input, false)
    }
}

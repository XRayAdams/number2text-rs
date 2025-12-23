use super::base_converter::BaseConverter;

pub struct FrenchConverter;

impl FrenchConverter {
    const ONES: [&'static str; 20] = [
        "zéro", "un", "deux", "trois", "quatre", "cinq", "six", "sept", "huit", "neuf",
        "dix", "onze", "douze", "treize", "quatorze", "quinze", "seize",
        "dix-sept", "dix-huit", "dix-neuf",
    ];

    const TENS: [&'static str; 10] = [
        "", "dix", "vingt", "trente", "quarante", "cinquante", "soixante", "soixante-dix",
        "quatre-vingt", "quatre-vingt-dix",
    ];
}

impl BaseConverter for FrenchConverter {
    fn name(&self) -> &str {
        "French"
    }

    fn native_number_too_large_error_text(&self) -> &str {
        "Nombre trop grand"
    }

    fn convert(&self, input: i64) -> String {
        if input > 999_999_999_999 {
            return self.native_number_too_large_error_text().to_string();
        }

        if input < 0 {
            return format!("moins {}", self.convert(-input));
        }

        if input < 20 {
            return Self::ONES[input as usize].to_string();
        }

        if input < 100 {
            let ten = input / 10;
            let unit = input % 10;

            if input == 80 {
                return "quatre-vingts".to_string();
            }

            if unit == 0 {
                return Self::TENS[ten as usize].to_string();
            }

            if ten == 7 || ten == 9 {
                if input == 71 {
                    return "soixante-et-onze".to_string();
                }
                return format!("{}-{}", Self::TENS[(ten - 1) as usize], Self::ONES[(10 + unit) as usize]);
            }

            if ten == 8 {
                return format!("{}-{}", Self::TENS[ten as usize], Self::ONES[unit as usize]);
            }

            if unit == 1 {
                return format!("{}-et-un", Self::TENS[ten as usize]);
            }

            return format!("{}-{}", Self::TENS[ten as usize], Self::ONES[unit as usize]);
        }

        if input < 1000 {
            let hundred = input / 100;
            let remainder = input % 100;

            let hundreds_str = if hundred == 1 {
                "cent".to_string()
            } else {
                format!("{} cent{}", Self::ONES[hundred as usize], if remainder == 0 { "s" } else { "" })
            };

            return if remainder == 0 {
                hundreds_str
            } else {
                format!("{} {}", hundreds_str, self.convert(remainder))
            };
        }

        if input < 1_000_000 {
            let thousands = input / 1000;
            let remainder = input % 1000;

            let thousands_str = if thousands == 1 {
                "mille".to_string()
            } else {
                format!("{} mille", self.convert(thousands))
            };

            return if remainder == 0 {
                thousands_str
            } else {
                format!("{} {}", thousands_str, self.convert(remainder))
            };
        }

        if input < 1_000_000_000 {
            let millions = input / 1_000_000;
            let remainder = input % 1_000_000;

            let millions_str = if millions == 1 {
                "un million".to_string()
            } else {
                format!("{} millions", self.convert(millions))
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
                "un milliard".to_string()
            } else {
                format!("{} milliards", self.convert(billions))
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

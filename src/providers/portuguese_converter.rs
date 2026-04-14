/*
    Copyright 2025 Konstantin Adamov
    Licenced under MIT Licence (https://opensource.org/licenses/MIT)
*/

use super::base_converter::BaseConverter;

pub struct PortugueseConverter;

impl PortugueseConverter {
    const ONES: [&'static str; 20] = [
        "zero", "um", "dois", "três", "quatro", "cinco", "seis", "sete", "oito", "nove",
        "dez", "onze", "doze", "treze", "catorze", "quinze", "dezasseis",
        "dezassete", "dezoito", "dezanove",
    ];

    const TENS: [&'static str; 10] = [
        "", "", "vinte", "trinta", "quarenta", "cinquenta", "sessenta", "setenta", "oitenta", "noventa",
    ];

    const HUNDREDS: [&'static str; 10] = [
        "", "cento", "duzentos", "trezentos", "quatrocentos", "quinhentos",
        "seiscentos", "setecentos", "oitocentos", "novecentos",
    ];
}

impl BaseConverter for PortugueseConverter {
    fn name(&self) -> &str {
        "Portuguese"
    }

    fn short_name(&self) -> &str {
        "pt"
    }

    fn native_number_too_large_error_text(&self) -> &str {
        "Número demasiado grande"
    }

    fn convert(&self, input: i64) -> String {
        if input > 999_999_999_999 {
            return self.native_number_too_large_error_text().to_string();
        }

        if input < 0 {
            return format!("menos {}", self.convert(-input));
        }

        if input < 20 {
            return Self::ONES[input as usize].to_string();
        }

        if input < 100 {
            let ten = input / 10;
            let unit = input % 10;
            return if unit == 0 {
                Self::TENS[ten as usize].to_string()
            } else {
                format!("{} e {}", Self::TENS[ten as usize], Self::ONES[unit as usize])
            };
        }

        if input < 1000 {
            let hundred = input / 100;
            let remainder = input % 100;

            if input == 100 {
                return "cem".to_string();
            }

            return if remainder == 0 {
                Self::HUNDREDS[hundred as usize].to_string()
            } else {
                format!("{} e {}", Self::HUNDREDS[hundred as usize], self.convert(remainder))
            };
        }

        if input < 1_000_000 {
            let thousands = input / 1000;
            let remainder = input % 1000;

            let thousands_str = if thousands == 1 {
                "mil".to_string()
            } else {
                format!("{} mil", self.convert(thousands))
            };

            return if remainder == 0 {
                thousands_str
            } else {
                format!("{} e {}", thousands_str, self.convert(remainder))
            };
        }

        if input < 1_000_000_000 {
            let millions = input / 1_000_000;
            let remainder = input % 1_000_000;

            let millions_str = if millions == 1 {
                "um milhão".to_string()
            } else {
                format!("{} milhões", self.convert(millions))
            };

            return if remainder == 0 {
                millions_str
            } else {
                format!("{} e {}", millions_str, self.convert(remainder))
            };
        }

        if input < 1_000_000_000_000 {
            let billions = input / 1_000_000_000;
            let remainder = input % 1_000_000_000;

            let billions_str = if billions == 1 {
                "mil".to_string()
            } else {
                format!("{} mil", self.convert(billions))
            };

            return if remainder == 0 {
                format!("{} milhões", billions_str)
            } else {
                format!("{} {}", billions_str, self.convert(remainder))
            };
        }

        self.native_number_too_large_error_text().to_string()
    }
}

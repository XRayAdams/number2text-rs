use super::base_converter::BaseConverter;

pub struct SpanishConverter;

impl SpanishConverter {
    const ONES: [&'static str; 20] = [
        "cero", "uno", "dos", "tres", "cuatro", "cinco", "seis", "siete", "ocho", "nueve",
        "diez", "once", "doce", "trece", "catorce", "quince", "dieciséis",
        "diecisiete", "dieciocho", "diecinueve",
    ];

    const TENS: [&'static str; 10] = [
        "", "", "veinte", "treinta", "cuarenta", "cincuenta", "sesenta", "setenta", "ochenta", "noventa",
    ];
}

impl BaseConverter for SpanishConverter {
    fn name(&self) -> &str {
        "Spanish"
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
                format!("{} y {}", Self::TENS[ten as usize], Self::ONES[unit as usize])
            };
        }

        if input < 1000 {
            let hundred = input / 100;
            let remainder = input % 100;

            if input == 100 {
                return "cien".to_string();
            }

            let hundreds_str = match hundred {
                1 => "ciento",
                5 => "quinientos",
                7 => "setecientos",
                9 => "novecientos",
                _ => return if remainder == 0 {
                    format!("{}cientos", Self::ONES[hundred as usize])
                } else {
                    format!("{}cientos {}", Self::ONES[hundred as usize], self.convert(remainder))
                },
            };

            return if remainder == 0 {
                hundreds_str.to_string()
            } else {
                format!("{} {}", hundreds_str, self.convert(remainder))
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
                format!("{} {}", thousands_str, self.convert(remainder))
            };
        }

        if input < 1_000_000_000 {
            let millions = input / 1_000_000;
            let remainder = input % 1_000_000;

            let millions_str = if millions == 1 {
                "un millón".to_string()
            } else {
                format!("{} millones", self.convert(millions))
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
                "mil".to_string()
            } else {
                format!("{} mil", self.convert(billions))
            };

            return if remainder == 0 {
                format!("{} millones", billions_str)
            } else {
                format!("{} {}", billions_str, self.convert(remainder))
            };
        }

        self.native_number_too_large_error_text().to_string()
    }
}

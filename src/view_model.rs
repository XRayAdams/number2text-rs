/*
    Copyright 2025 Konstantin Adamov
    Licenced under MIT Licence (https://opensource.org/licenses/MIT)
*/

use crate::providers::number_converter::NumberConverter;

pub struct AppViewModel {
    number_converter: NumberConverter,
    selected_index: usize,
}

impl AppViewModel {
    pub fn new() -> Self {
        Self {
            number_converter: NumberConverter::new(),
            selected_index: 0,
        }
    }

    /// Returns the list of available language names.
    pub fn get_language_names(&self) -> Vec<String> {
        self.number_converter
            .base_converters
            .iter()
            .map(|c| c.name().to_string())
            .collect()
    }

    pub fn convert_number(&self, input: i64) -> String {
        self.number_converter.base_converters[self.selected_index].convert(input)
    }

    pub fn set_language(&mut self, index: usize) {
        if index < self.number_converter.base_converters.len() {
            self.selected_index = index;
        }
    }

    pub fn get_app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn convert_by_language(&self, language_code: &str, input: i64) -> Option<String> {
        for converter in self.number_converter.base_converters.iter() {
            if converter.short_name() == language_code {
                return Some(converter.convert(input));
            }
        }
        None
    }
}

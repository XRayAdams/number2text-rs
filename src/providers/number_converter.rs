/*
    Copyright 2025 Konstantin Adamov
    Licenced under MIT Licence (https://opensource.org/licenses/MIT)
*/

use super::base_converter::BaseConverter;
use super::english_converter::EnglishConverter;
use super::french_converter::FrenchConverter;
use super::german_converter::GermanConverter;
use super::italian_converter::ItalianConverter;
use super::portuguese_converter::PortugueseConverter;
use super::russian_converter::RussianConverter;
use super::spanish_converter::SpanishConverter;

pub struct NumberConverter {
    pub base_converters: Vec<Box<dyn BaseConverter>>,
}

impl NumberConverter {
    pub fn new() -> Self {
        Self {
            base_converters: vec![
                Box::new(EnglishConverter),
                Box::new(FrenchConverter),
                Box::new(GermanConverter),
                Box::new(ItalianConverter),
                Box::new(PortugueseConverter),
                Box::new(RussianConverter),
                Box::new(SpanishConverter),
            ],
        }
    }
}

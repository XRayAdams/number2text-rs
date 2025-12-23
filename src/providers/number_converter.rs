use super::base_converter::BaseConverter;
use super::english_converter::EnglishConverter;
use super::spanish_converter::SpanishConverter;
use super::french_converter::FrenchConverter;
use super::german_converter::GermanConverter;
use super::italian_converter::ItalianConverter;
use super::russian_converter::RussianConverter;

pub struct NumberConverter {
    pub base_converters: Vec<Box<dyn BaseConverter>>,
}

impl NumberConverter {
    pub fn new() -> Self {
        Self {
            base_converters: vec![
                Box::new(EnglishConverter),
                Box::new(SpanishConverter),
                Box::new(FrenchConverter),
                Box::new(GermanConverter),
                Box::new(ItalianConverter),
                Box::new(RussianConverter),
            ],
        }
    }
}

impl Default for NumberConverter {
    fn default() -> Self {
        Self::new()
    }
}

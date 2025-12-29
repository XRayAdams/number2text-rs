/*
    Copyright 2025 Konstantin Adamov
    Licenced under MIT Licence (https://opensource.org/licenses/MIT)
*/


/// BaseConverter trait for number-to-text conversion providers.
pub trait BaseConverter {
    /// Returns the name of the language.
    fn name(&self) -> &str;
    /// Returns the short name (language code) of the language.
    fn short_name(&self) -> &str;
    /// Returns the error text for numbers that are too large to convert.
    fn native_number_too_large_error_text(&self) -> &str;
    /// Converts the given number to its textual representation.
    fn convert(&self, input: i64) -> String;
}

pub trait BaseConverter {
    fn name(&self) -> &str;
    fn native_number_too_large_error_text(&self) -> &str;
    fn convert(&self, input: i64) -> String;
}

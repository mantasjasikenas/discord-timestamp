use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref DATE_TIME_REGEX: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}(T|\s)\d{2}:\d{2}(:\d{2})?(\.\d{1,6})?$").unwrap();
    pub static ref TIME_REGEX: Regex = Regex::new(r"^\d{2}:\d{2}(:\d{2})?(\.\d{1,6})?$").unwrap();
    pub static ref OFFSET_REGEX: Regex = Regex::new(r"([+-]\d+[smhd])").unwrap();
}
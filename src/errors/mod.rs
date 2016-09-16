// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

// utf8 to str error.
pub fn utf8_to_str_error() -> String {
    "Error converting utf8 to str".to_owned()
}


// str to bool error.
pub fn str_to_bool_error() -> String {
    "Error converting str to bool".to_owned()
}


// str to datetime error.
pub fn str_to_datetime_error() -> String {
    "Error converting str to datetime".to_owned()
}


// str to i64 error.
pub fn str_to_i64_error() -> String {
    "Error converting str to i64".to_owned()
}


// element to string error.
pub fn element_to_string_error() -> String {
    "Error converting element to string".to_owned()
}


// response error
pub fn missing_xml_error() -> String {
    "Url must end with .xml".to_owned()
}


// item required field error
pub fn item_required_field_error() -> String {
    "Either Title or Description must have a value".to_owned()
}


// xml start tag error
pub fn tag_start_error(tag: &str) -> String {
    format!("Error creating start tag for {}", tag)
}


// xml start tag error
pub fn tag_text_error(tag: &str) -> String {
    format!("Error creating text for {}", tag)
}


// xml end tag error
pub fn tag_end_error(tag: &str) -> String {
    format!("Error creating end tag for {}", tag)
}


// negative integer error
pub fn negative_error(tag: &str, num: i64) -> String {
    format!("{} cannot be negative: {}", tag, num)
}


// invalid integer error
pub fn invalid_int_error(tag: &str, num: i64) -> String {
    format!("{} contains an invalid value: {}", tag, num)
}


// invalid string error
pub fn invalid_str_error(tag: &str, string: &str) -> String {
    format!("{} contains an invalid value: {}", tag, string)
}


// content type error
pub fn content_type_error() -> String {
    "Error retrieving content type".to_owned()
}

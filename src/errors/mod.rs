// Copyright (c) 2016 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the LGPLv3 license that can be
// found in the LICENSE file.

// utf8 to str error.
pub fn utf8_to_str_error() -> &'static str {
    "Error converting utf8 to str"
}


// str to bool error.
pub fn str_to_bool_error() -> &'static str {
    "Error converting str to bool"
}


// str to datetime error.
pub fn str_to_datetime_error() -> &'static str {
    "Error converting str to datetime"
}


// str to i64 error.
pub fn str_to_i64_error() -> &'static str {
    "Error converting str to i64"
}


// element to string error.
pub fn element_to_string_error() -> &'static str {
    "Error converting element to string"
}


// response error
pub fn response_error() -> &'static str {
    "Error retrieving response"
}


// response error
pub fn missing_xml_error() -> &'static str {
    "Url must end with .xml"
}


// empty feed error
pub fn empty_feed_error() -> &'static str {
    "Feed is empty"
}


// item required field error
pub fn item_required_field_error() -> &'static str {
    "Either Title or Description must have a value"
}

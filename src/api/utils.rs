use crate::api::constants::{ENTER_STR, ENTER_STR2};

pub fn delete_enters(input: &mut String) -> String {
    let mut output: String = String::new();
    for i in input.chars() {
        if i.to_string() != ENTER_STR && i.to_string() != ENTER_STR2 {
            output.push_str(&*i.to_string());
        }
    }
    output
}

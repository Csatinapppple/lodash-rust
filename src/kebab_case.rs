extern crate regex;

///  Converts `String` to [kebab case](https://en.wikipedia.org/wiki/Letter_case#Special_case_styles).
///
/// @category String
/// @param {String} [string=''] The string to convert.
/// @returns {String} Returns the kebab cased string.
///

pub fn new(s: &str) -> String {
    let re = regex::Regex::new("[^a-zA-Z0-9]+").unwrap();
    let result = re.replace_all(s, " ");
    let result = result.trim();
    let result_string = re.replace_all(result, "-");

    // check if string contains "-"
    let re2 = regex::Regex::new("[-]+").unwrap();
    let contains_hyphen = re2.is_match(result_string.as_ref());

    let mut build_result_string = String::new();

    if !contains_hyphen {
        let mut tmp = [0u8; 4];
        // convert to char
        let characters: Vec<char> = result_string.chars().collect();
        for letter in characters {
            if letter.is_uppercase() {
                build_result_string.push_str("-");
            }
            build_result_string.push_str(letter.encode_utf8(&mut tmp));
        }

        return build_result_string.to_lowercase();
    }

    return result_string.to_string().to_lowercase();
}

#[test]
fn test_new() {
    assert_eq!(new("Foo Bar"), "foo-bar");
    assert_eq!(new("fooBar"), "foo-bar");
    assert_eq!(new("__FOO_BAR__"), "foo-bar");
}

use obfustring::obfustring;

#[test]
fn utf8_string_validity() {
    let obfuscated_string = obfustring!("Hello obfustring!");
    let generic_string = String::from("Hello obfustring!");
    let generic_str = "Hello obfustring!";

    assert_eq!(obfuscated_string, generic_string);
    assert_eq!(obfuscated_string, generic_str);
}

#[test]
fn utf16_string_validity() {
    let obfuscated_string = obfustring!("你好迷惑！, but it includes utf-16!");
    let generic_string = String::from("你好迷惑！, but it includes utf-16!");
    let generic_str = "你好迷惑！, but it includes utf-16!";

    assert_eq!(obfuscated_string, generic_string);
    assert_eq!(obfuscated_string, generic_str);
}

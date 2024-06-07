use regex::Regex;

pub fn validate_email(email: &str) -> bool {
    Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)])"#).unwrap().is_match(email)
}

pub fn validate_username(username: &str) -> bool {
    Regex::new(r"^[\p{L}\s]+$").unwrap().is_match(username)
}

pub fn validate_password(password: &str) -> bool {
    // Check length
    if password.len() < 8 {
        return false;
    }

    // Check lowercase letters
    let lowercase_regex = Regex::new(r"[a-z]").unwrap();
    if !lowercase_regex.is_match(password) {
        return false;
    }

    // Check uppercase letters
    let uppercase_regex = Regex::new(r"[A-Z]").unwrap();
    if !uppercase_regex.is_match(password) {
        return false;
    }

    // Check digits
    let digit_regex = Regex::new(r"\d").unwrap();
    if !digit_regex.is_match(password) {
        return false;
    }

    // Check special characters
    let special_regex = Regex::new(r"[@$!%*?&]").unwrap();
    if !special_regex.is_match(password) {
        return false;
    }

    true
}

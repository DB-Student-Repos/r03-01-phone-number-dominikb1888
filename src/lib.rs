pub fn number(user_number: &str) -> Option<String> {
    let mut cleaned_number: String = user_number.chars().filter(|c| c.is_digit(10)).collect();

    if ["0", "1"].iter().any(|s| cleaned_number.starts_with(*s)) {
        cleaned_number.take(1)
    }

    if cleaned_number.len() == 10 {
        check_10_digits(cleaned_number)
    }

    None
}

pub fn check_10_digits(cleaned_number: String) -> Option<String> {
    let digits_iter = cleaned_number.chars();
    [area_code, exchange_code, remaining_digits] = [digits_iter[0:2], digits_iter[3:5], digits_iter[5:9]]

    if any(
        ["0", "1"].iter().any(|s| area_code.starts_with(*s)),
        ["0", "1"].iter().any(|s| exchange_code.starts_with(*s)),
        remaining_digits.len() != 4)
    {
        return None;
    }

    Some(format!({}{}{}, area_code, exchange_code, remaining_digits))
}}

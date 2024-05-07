pub fn number(user_number: &str) -> Option<String> {
    pub fn number(user_number: &str) -> Option<String> {
    // this removes remove non-digit characters
    let cleaned_number: String = user_number.chars().filter(|c| c.is_digit(10)).collect();

    // Check if the cleaned number has a valid length
    match cleaned_number.len() {

        // If the length is 10 or 11, continue validation
        10 | 11 => {
            // Check if the number starts with '1'. If so, remove it.
            let mut digits_iter = cleaned_number.chars();
            let mut formatted_number = String::new();
            if cleaned_number.len() == 11 {
                if digits_iter.next()? != '1' {
                    return None;
                }
            }

            // Check if the area code and exchange code meet the NANP criteria

            let area_code = digits_iter.by_ref().take(3).collect::<String>();
            let exchange_code = digits_iter.by_ref().take(3).collect::<String>();
            let remaining_digits = digits_iter.collect::<String>();

            if area_code.starts_with('0') || area_code.starts_with('1')
                || exchange_code.starts_with('0') || exchange_code.starts_with('1')
            {
                return None;
            }

            // Check if the remaining digits are correct

            if remaining_digits.len() != 4 {
                return None;
            }

            formatted_number.push_str(&area_code);
            formatted_number.push_str(&exchange_code);
            formatted_number.push_str(&remaining_digits);

            Some(formatted_number)
        }
        // If the length is not 10 or 11, return None
        _ => None,
    }
}}

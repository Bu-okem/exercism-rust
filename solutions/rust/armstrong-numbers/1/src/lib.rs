pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let num_of_digits: usize = digits.len();
    let mut sum: u32 = 0;
    
    for digit in digits.chars() {
        let digit_char: u32 =  digit.to_digit(10).expect("Not char");
        sum = sum + (digit_char.pow(num_of_digits as u32));
    }

    sum == num
}

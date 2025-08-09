fn is_prime(x: u32) -> bool {
    if x < 2 {
        false 
    } else {
        for number in 2..=x.isqrt() {
            if x % number == 0 {
                return false;
            }
        }

        true
    }
}

pub fn nth(n: i32) -> u32 {
    let mut counter: i32 = -1;
    let mut number: u32 = 1;
    while counter < n {
        number += 1;
        if is_prime(number) {
            counter += 1;
        }
    }

    number
}

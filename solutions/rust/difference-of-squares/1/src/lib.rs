pub fn square_of_sum(n: u32) -> u32 {
    let mut count: u32 = 0;
    let mut sum: u32 = 0;
    while count <= n {
        sum += count;
        count += 1;
    }

    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut count: u32 = 0;
    let mut sum: u32 = 0;
    while count <= n {
        let num_pow = count.pow(2);
        sum += num_pow;
        count += 1;
    }

    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

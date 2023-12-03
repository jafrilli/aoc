pub fn exec(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let first_digit = line.chars()
            .find(|c| c.is_digit(10))
            .and_then(|c| c.to_digit(10))
            .unwrap();

        let second_digit = line.chars()
            .rev()
            .find(|c| c.is_digit(10))
            .and_then(|c| c.to_digit(10))
            .unwrap();

        sum += (first_digit * 10) + second_digit;
    }
    sum.to_string()
}

// #202
pub fn is_happy(n: i32) -> bool {
    let mut sum:i32 = 0;
    let mut cache:Vec<i32> = Vec::new();

    fn sum_digit_squares(value:String, mut sum: i32,  cache: &mut Vec<i32>) -> i32 {
        sum=0;
        for c in value.chars() {
            sum += i32::pow(c.to_digit(10).unwrap() as i32, 2);
        }
        if cache.contains(&sum) { return -1; }
        cache.push(sum);
        return sum;
    }

    let mut result = sum_digit_squares(n.to_string(), sum, &mut cache);
    loop {
        if result == 1 { return true ; }
        else if result == -1 { return false ; }
        else { result = sum_digit_squares(result.to_string(), sum, &mut cache); }
    }
}

#[cfg(test)]
mod test {
    use crate::Day1::HappyNumber::is_happy;

    #[test]
    pub fn test1() {
        assert_eq!(is_happy(100), true)
    }
}
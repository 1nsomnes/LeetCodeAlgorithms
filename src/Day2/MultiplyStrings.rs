pub fn multiply(num1: String, num2: String) -> String {

    todo!();
}

#[cfg(test)]
mod test {
    use crate::Day2::MultiplyStrings::multiply;

    #[test]
    pub fn test1() {
        assert_eq!(multiply(String::from("123"), String::from("456")), String::from("56088"));
    }
}


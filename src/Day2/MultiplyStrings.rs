// #43
pub fn multiply(num1: String, num2: String) -> String {

    struct Number {
        value: u64,
        place:u32,
    }

    impl Number {
        fn new(value:u64, place:u32) -> Self {
            Self {
                value,
                place
            }
        }

        fn multiply_to_string(&self, num2:&Number) -> String {
            let multiplied = self.value * num2.value;
            let total_places = self.place + num2.place;
            let mut result = multiplied.to_string();

            for _ in 0..total_places { result.push('0');}

            result
        }
    }

    pub fn add_strings(num1: String, num2: String) -> String {
        let mut numbers:(Vec<char>, Vec<char>) = if num1.len() > num2.len() {
            (num1.chars().collect(), num2.chars().collect())
        } else {
            (num2.chars().collect(),num1.chars().collect())
        };
        numbers = (numbers.0.into_iter().rev().collect::<Vec<char>>(), numbers.1.into_iter().rev().collect::<Vec<char>>());
        let mut result:String = String::from("");
        let mut carry:u32 = 0;

        for i in 0..numbers.0.len() {
            let mut sum:u32;
            if i as usize >= numbers.1.len() {
                sum = (numbers.0.get(i).unwrap().to_digit(10).unwrap() + carry) as u32
            } else {
                sum = (numbers.0.get(i).unwrap().to_digit(10).unwrap() + numbers.1.get(i).unwrap().to_digit(10).unwrap() + carry) as u32;
            };

            if sum >= 10 {
                if i >= numbers.0.len()-1 {
                    let reversed = sum.to_string().chars().rev().collect::<String>();
                    result.push_str(reversed.as_str());
                } else {
                    carry = 1;
                    sum -= 10;
                    result.push_str(sum.to_string().as_str());
                }
            } else {
                carry=0;
                result.push_str(sum.to_string().as_str());
            }

        }

        result.chars().rev().collect::<String>()
    }

    fn split_string(num:String, size:usize) -> Vec<Number> {
        let characters:Vec<char> = num.chars().rev().collect();
        let char_chunks = characters.chunks(size);

        let mut result:Vec<Number> = Vec::new();
        let mut place:u32 = 0;

        char_chunks.for_each(|c| {
            let num = c.iter().rev().collect::<String>();
            result.push(Number::new(num.parse::<u64>().unwrap(),place));

            place += size as u32;
        });
        result
    }

    if num1 == "0" || num2 == "0" { return String::from("0"); }

    let mut numbers:Vec<String> = Vec::new();

    let mut top = split_string(num1,7);
    let mut bot = split_string(num2, 7);

    for b in bot {
        for t in &top {
            numbers.push(b.multiply_to_string(t));
        }
    }

    let mut result = numbers[0].clone();

    for i in 1..numbers.len() {
        result = add_strings(result, numbers[i].clone());
    }

    result
}

#[cfg(test)]
mod test {
    use crate::Day2::MultiplyStrings::multiply;

    #[test]
    pub fn test1() {

        assert_eq!(multiply(String::from("123"), String::from("456")), String::from("56088"));
    }

}


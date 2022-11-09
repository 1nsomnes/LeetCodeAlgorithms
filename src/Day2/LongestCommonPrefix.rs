pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result = String::new();
    let strs:Vec<Vec<char>> = strs.into_iter().map(|word| word.chars().collect()).collect();
    let mut index = 0;

    loop {
        if index >= strs[0].len() { return result; }
        let currChar:char = strs[0][index];

        for word_index in 1..strs.len() {
            if index >= strs[word_index].len() || strs[word_index][index] != currChar { return result; }
        }
        result.push(currChar);
        index += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::Day2::LongestCommonPrefix::longest_common_prefix;

    #[test]
    pub fn test1() {
        assert_eq!(longest_common_prefix(vec![String::from("flower"),String::from("flow"),String::from("flight")]), String::from("fl"));
        assert_eq!(longest_common_prefix(vec![String::from("dog"),String::from("racecar"),String::from("car")]), String::from(""));
    }
}
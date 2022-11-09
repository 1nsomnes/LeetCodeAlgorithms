pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut x:usize = 0;
    let mut y:usize = 0;

    let mut left_bound:i32 = -1;
    let mut upper_bound:i32 = -1;
    let mut right_bound:i32 = matrix[0].len() as i32;
    let mut lower_bound:i32 = matrix.len() as i32;

    let mut value:Vec<i32> = Vec::new();

    loop {
        for i in (left_bound + 1)..right_bound {
            x = i as usize;
            value.push(matrix[y][x]);
        }
        upper_bound += 1;
        if upper_bound + 1 == lower_bound { return value; }

        for i in (upper_bound + 1)..lower_bound {
            y = i as usize;
            value.push(matrix[y][x]);
        }
        right_bound -= 1;
        if left_bound + 1 == right_bound { return value; }


        for i in (left_bound + 1..right_bound).rev() {
            x = i as usize;
            value.push(matrix[y][x]);
        }
        lower_bound -= 1;
        if upper_bound + 1 == lower_bound { return value; }

        for i in (upper_bound + 1..lower_bound).rev() {
            y = i as usize;
            value.push(matrix[y][x]);
        }
        left_bound += 1;
        if left_bound + 1 == right_bound { return value; }
    }
}

#[cfg(test)]
mod tests {
    use crate::Day1::SpiralMatrix::spiral_order;

    #[test]
    fn test() {
        let matrix = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
        let answer = vec![1,2,3,4,8,12,11,10,9,5,6,7];

        assert_eq!(spiral_order((matrix)), answer);

        let matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        let answer = vec![1,2,3,6,9,8,7,4,5];

        assert_eq!(spiral_order((matrix)), answer);
    }
}
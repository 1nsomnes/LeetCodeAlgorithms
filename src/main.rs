use crate::Day1::HappyNumber::is_happy;
use crate::Day1::SpiralMatrix::spiral_order;

mod Day1 {
    pub mod HappyNumber;
    pub mod SpiralMatrix;
    pub mod WhereWillTheBallFall;
}

fn main() {
    let matrix = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
    spiral_order(matrix);
    println!("Hello, world!");
}

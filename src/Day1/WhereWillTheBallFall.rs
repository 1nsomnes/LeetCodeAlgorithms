// #1706
pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result:Vec<i32> = Vec::new();

    for ball in 0..grid[0].len() {
        let mut y = 0;
        let mut x = ball as i32;
        println!("{}", ball);
        loop {
            if grid[y as usize][x as usize] == 1 {
                x += 1;
                if x < 0 || x >= grid[0].len() as i32 { result.push(-1); break; }
                if grid[y as usize][x as usize] == 1 {
                    y += 1;
                    if y >= grid.len() { result.push(x); break;}
                } else {
                    result.push(-1);
                    break;
                }
            }

            if grid[y as usize][x as usize] == -1 {
                x -= 1;
                if x < 0 || x >= grid[0].len() as i32 { result.push(-1); break; }
                if grid[y as usize][x as usize] == -1 {
                    y += 1;
                    if y >= grid.len() { result.push(x); break;}
                } else {
                    result.push(-1);
                    break;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::Day1::WhereWillTheBallFall::find_ball;

    #[test]
    fn test() {
        let grid = vec![vec![1,1,1,-1,-1],vec![1,1,1,-1,-1],vec![-1,-1,-1,1,1],vec![1,1,1,1,-1],vec![-1,-1,-1,-1,-1]];
        let answer = vec![1,-1,-1,-1,-1];

        assert_eq!(find_ball(grid), answer);
    }
}
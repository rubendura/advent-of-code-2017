use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let maze: Vec<i32> = input.trim().lines().map(|l| l.parse::<i32>().expect("Not a number")).collect();
    let result = maze_exit_steps(&maze);
    println!("{}", result);
}

fn maze_exit_steps(maze: &[i32]) -> i32 {
    let current = 0;
    let count = 0;
    loop {
        if current > maze.len() {
            break count;
        }

        let instruction = maze[current];
        maze[current] += 1;
        current += instruction;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![0,3,0,1,-3];
        let expected = 5;
        let result = maze_exit_steps(&input);
        assert_eq!(expected, result);
    }
}
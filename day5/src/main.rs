use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let maze: Vec<i32> = input.trim().lines().map(|l| l.parse::<i32>().expect("Not a number")).collect();
    let result = maze_exit_steps(&mut maze.clone(), incrementer2);
    println!("{}", result);
}


fn incrementer(input: i32) -> i32 { input + 1 }
fn incrementer2(input: i32) -> i32 {
    if input >= 3 {
        input - 1
    }
    else {
        incrementer(input)
    }
}

fn maze_exit_steps<T: Fn(i32) -> i32>(maze: &mut [i32], inrementer: T) -> u32 {
    let mut current: u32 = 0;
    let mut count = 0;
    loop {
        if current >= maze.len() as u32 {
            return count;
        }

        let instruction = maze[current as usize];
        maze[current as usize] = inrementer(instruction);
        current = ((current as i32) + instruction) as u32;
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![0,3,0,1,-3];
        let expected = 5;
        let result = maze_exit_steps(&mut input.clone(), incrementer);
        assert_eq!(expected, result);
    }

    #[test]
    fn test2() {
        let input = vec![0,3,0,1,-3];
        let mut input_clone = input.clone();
        let expected = 10;
        let result = maze_exit_steps(&mut input_clone, incrementer2);
        assert_eq!(input, vec![0,3,0,1,-3]);
        assert_eq!(expected, result);
        assert_eq!(input_clone, vec![2, 3, 2, 3, -1]);
    }
}

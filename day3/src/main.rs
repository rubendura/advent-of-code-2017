use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Not UTF-8");
    let input: u32 = input.parse().expect("Not an unsigned number");
    let result = required_steps(input as i32);
    println!("{}", result);
}

fn required_steps(input: i32) -> i32 {

    if input == 1 {
        return 0;
    }

    // Find pivot (highest odd n such that n*n < input)
    let mut current = 1;
    let pivot = loop {
        let next = current + 2;
        if (next * next) > input {
            break current;
        }
        else {
            current = next;
        }
    };
    println!("Pivot: {}", pivot);

    // Compute "edge_central" element (matrix[n-1][(n/2)+1] element)
    let edge_central = (pivot * pivot) + pivot / 2 + 1;
    println!("Edge central: {}", edge_central);

    // Compute input-central % ( pivot/2 + 1) -> steps from position till central
    // Compute pivot / 2 (floor) -> steps from central to 1
    // Add
    let steps_to_edge_central = ((input - edge_central) % (pivot / 2 + 1)).abs();
    println!("Steps to edge central: {}", steps_to_edge_central);
    let steps_to_centre_from_edge = pivot / 2 + 1;
    println!("Steps to centre: {}", steps_to_centre_from_edge);

    let required_steps = steps_to_edge_central + steps_to_centre_from_edge;
    println!("Total steps: {}", required_steps);

    required_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = 1;
        let expected = 0;
        let result = required_steps(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn example2() {
        let input = 12;
        let expected = 3;
        let result = required_steps(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn example3() {
        let input = 23;
        let expected = 2;
        let result = required_steps(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn example4() {
        let input = 1024;
        let expected = 31;
        let result = required_steps(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn extra_example1() {
        let input = 10;
        let expected = 3;
        let result = required_steps(input);
        assert_eq!(expected, result);
    }
}
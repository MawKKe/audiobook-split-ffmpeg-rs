fn main() {
    println!("Hello, world!");
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_add() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn internal_add() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

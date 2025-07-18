pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(x: i32) -> i32 {
    x+2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}

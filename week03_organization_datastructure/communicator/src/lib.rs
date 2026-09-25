pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod network {
    fn connect() {

    }
}

mod client{
    fn connect() {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

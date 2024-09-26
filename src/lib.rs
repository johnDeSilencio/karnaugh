pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[allow(unused)]
pub fn generate(num_bits: u8) -> Vec<u128> {
    unimplemented!()
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

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

    #[test]
    fn zero_bit_k_map() {
        // Arrange
        let num_bits = 0u8;
        let expected_k_map = [0u128; 0];

        // Act
        let k_map = generate(num_bits);

        // Assert
        assert_eq!(k_map, expected_k_map);
    }
}

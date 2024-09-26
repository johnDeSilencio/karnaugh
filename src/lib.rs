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

    #[test]
    fn one_bit_k_map() {
        // Arrange
        let num_bits = 1u8;
        let expected_k_map = [0b0, 0b1];

        // Act
        let k_map = generate(num_bits);

        // Assert
        assert_eq!(k_map, expected_k_map);
    }

    #[test]
    fn two_bit_k_map() {
        // Arrange
        let num_bits = 2u8;
        let expected_k_map = [
            0b00, 0b01, //
            0b10, 0b11, //
        ];

        // Act
        let k_map = generate(num_bits);

        // Assert
        assert_eq!(k_map, expected_k_map);
    }

    #[test]
    fn three_bit_k_map() {
        // Arrange
        let num_bits = 3u8;
        let expected_k_map = [
            0b000, 0b001, 0b101, 0b100, //
            0b010, 0b011, 0b111, 0b110, //
        ];

        // Act
        let k_map = generate(num_bits);

        // Assert
        assert_eq!(k_map, expected_k_map);
    }

    #[test]
    fn four_bit_k_map() {
        // Arrange
        let num_bits = 4u8;
        let expected_k_map = [
            0b0000, 0b0001, 0b0101, 0b0100, //
            0b0010, 0b0011, 0b0111, 0b0110, //
            0b1010, 0b1011, 0b1111, 0b1110, //
            0b1000, 0b1001, 0b1101, 0b1100, //
        ];

        // Act
        let k_map = generate(num_bits);

        // Assert
        assert_eq!(k_map, expected_k_map);
    }
}

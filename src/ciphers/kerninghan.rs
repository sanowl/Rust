pub fn kerninghan(mut n: u32) -> u32 {
    let mut count = 0;

    while n > 0 {
        n &= n - 1;
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_set_bits() {
        assert_eq!(kerninghan(0b0000_0000_0000_0000_0000_0000_0000_1011), 3);
        assert_eq!(kerninghan(0b0000_0000_0000_0000_0000_0000_1000_0000), 1);
        assert_eq!(kerninghan(0b1111_1111_1111_1111_1111_1111_1111_1101), 31);
    }
}

// Magic Table to enable fast implementation of a bit scan function
static MOD67TABLE: [usize; 67] = [
    64, 0, 1, 39, 2, 15, 40, 23,
    3, 12, 16, 59, 41, 19, 24, 54,
    4, 64, 13, 10, 17, 62, 60, 28,
    42, 30, 20, 51, 25, 44, 55, 47,
    5, 32, 64, 38, 14, 22, 11, 58,
    18, 53, 63, 9, 61, 27, 29, 50,
    43, 46, 31, 37, 21, 57, 52, 8,
    26, 49, 45, 36, 56, 7, 48, 35,
    6, 34, 33];

// turns the first non-zero bit in a u64
pub fn bit_scan(bit: u64) -> usize {
    let remainder = (bit % 67) as usize;
    return MOD67TABLE[remainder];
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_scan_works(){
        for i in 0..64 {
            let bit = (1 as u64) << i;
            let index = bit_scan(bit);
            assert_eq!(i, index);
        }
    }
    #[test]
    #[should_panic]
    fn bit_scan_works_highest_bit_is_1(){
        for i in 0..64 {
            let mut bit = (1 as u64) << i;
            bit |= (1 as u64) << 63;
            let index = bit_scan(bit);
            assert_eq!(i, index);
        }
    }
}

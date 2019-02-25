//! Bit Manipulation
//! ================
//!
//! Here are bitwise operations in Rust.
//!
//! | Operation   | Notation |
//! |:------------|:--------:|
//! | NOT         | `!`      |
//! | AND         | `&`      |
//! | OR          | `\|`     |
//! | XOR         | `^`      |
//! | Right shift | `>>`     |
//! | Left shift  | `<<`     |
//!
//! We can specify data in binary format using `0b`-prefix.
//!
//! ## Examples
//!
//! ```
//! assert_eq!(12, 0b1100);
//! assert_eq!(-1i8, i8::from_be_bytes([0b11111111]));
//! assert_eq!(0b1000, 0b0110 + 0b0010);
//! assert_eq!(0b0011, 0b1100 >> 2);
//! assert_eq!(-1i8, 0b1101 ^ (!0b1101));
//! assert_eq!(0b1000, 0b1101 ^ 0b0101);
//! assert_eq!(0b1000, 0b1011 & (!0 << 2));
//! ```

/// Check if i-th bit of n is 1 or not.
///
/// ## Examples
///
/// ```
/// use ctci_6th_edition::ch5::get_bit;
///
/// assert!(get_bit(-1, 0));
/// assert!(!get_bit(0, 0));
/// ```
pub fn get_bit(n: isize, i: usize) -> bool {
    (n & (1 << i)) != 0
}

/// Set i-th bit of n to 1.
///
/// ## Examples
///
/// ```
/// use ctci_6th_edition::ch5::set_bit;
///
/// assert_eq!(1, set_bit(0, 0));
/// ```
pub fn set_bit(n: isize, i: usize) -> isize {
    n | (1 << i)
}

/// Clear i-th bit of n.
///
/// ## Examples
///
/// ```
/// use ctci_6th_edition::ch5::clear_bit;
///
/// assert_eq!(0, clear_bit(256, 8));
/// ```
pub fn clear_bit(n: isize, i: usize) -> isize {
    let mask = !(1 << i);
    n & mask
}

/// Clear all bits of num from the most significant bit through i (inclusive).
///
/// ## Examples
///
/// ```
/// use ctci_6th_edition::ch5::clear_bits_msb_through_i;
///
/// assert_eq!(isize::max_value() / 2 + 1, clear_bits_msb_through_i(-1, 63));
/// ```
pub fn clear_bits_msb_through_i(num: isize, i: usize) -> isize {
    let mask = 1 << i - 1;
    num & mask
}

/// Clear all bits of num from i through 0 (inclusive).
///
/// ## Examples
///
/// ```
/// use ctci_6th_edition::ch5::clear_bits_i_through_0;
///
/// assert_eq!(-8, clear_bits_i_through_0(-1, 2));
/// ```
pub fn clear_bits_i_through_0(num: isize, i: usize) -> isize {
    let mask = !0 << (i + 1);
    num & mask
}

/// Set i-th bit of n following bit_is_1.
///
/// ## Examples
///
/// ```
/// use ctci_6th_edition::ch5::update_bit;
///
/// assert_eq!(256, update_bit(0, 8, true));
/// ```
pub fn update_bit(num: isize, i: usize, bit_is_1: bool) -> isize {
    let mask = !(1 << i);
    num & mask | isize::from(bit_is_1) << i
}

mod insertion;
pub use self::insertion::*;

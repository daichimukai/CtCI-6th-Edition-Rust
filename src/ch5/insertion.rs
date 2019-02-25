/// Given two 32-bit numbers `n` and `m`, return the number `m` merged into `n`.
///
/// ## Examples
/// ```
/// use ctci_6th_edition::ch5::insertion;
///
/// assert_eq!(0b10001001100, insertion(0b10000000000, 0b10011, 2, 6));
/// ```
pub fn insertion(n: u32, m: u32, i: u32, j: u32) -> u32 {
    let m_shifted = m << i;
    let mask = !((1 << (j - i + 2)) - 1 << i);
    n & mask | m_shifted
}

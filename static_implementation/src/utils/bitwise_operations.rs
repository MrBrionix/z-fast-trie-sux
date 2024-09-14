pub fn get_fattest(x: usize, low: usize) -> usize {
    assert!(x >= low);
    let calc = |i: usize| -> usize { !(((i + 1).next_power_of_two() >> 1) - 1) };

    if low == 0 {
        0
    } else {
        let m = calc((low - 1) ^ x);
        m & x
    }
}

pub fn calc(i: usize) -> usize {
    !((i + 1).next_power_of_two() - 1)
}

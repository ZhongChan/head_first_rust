fn main() {
    u8_overflow();
}

/*
整数溢出
wrapping_* 方法：补码循环溢出
checked_* 方法: None
overflowing_* 方法：是否溢出标志位
saturating_* 方法：计算结果在指定范围内[min,max]

debug:报错
release:循环处理 256->0 257->1 。程序是错的
*/
fn u8_overflow() {
    assert_eq!(255u8.wrapping_add(20), 19);

    assert_eq!(255u8.checked_add(20), None);
    assert_eq!(100u8.checked_add(20), Some(120));

    assert_eq!(255u8.overflowing_add(20), (19, true));
    assert_eq!(100u8.overflowing_add(20), (120, false));

    assert_eq!(100u8.saturating_add(1), 101);
    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);
}
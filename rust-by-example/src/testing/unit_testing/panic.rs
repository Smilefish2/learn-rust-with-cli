/**
一些函数应当在特定条件下 panic。为测试这种行为，请使用 #[should_panic] 属性。
这 个属性接受可选参数 expected = 以指定 panic 时的消息。
如果你的函数能以多种方式 panic，这个属性就保证了你在测试的确实是所指定的 panic。
*/
pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}
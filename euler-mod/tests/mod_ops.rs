use euler_mod::mul_mod;

#[test]
fn test_mul_mod() {
    let a: u64 = 123456789;
    let b = 987654321;
    let m = 1_000_000_007;
    assert_eq!(mul_mod(a, b, m), 123456789 * 987654321 % m);
}

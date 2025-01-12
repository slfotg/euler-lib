pub trait ModOps:
    Copy
    + PartialOrd
    + PartialEq
    + num_traits::ConstZero
    + num_traits::ConstOne
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::ops::RemAssign
    + std::ops::BitAnd<Output = Self>
    + std::ops::Shr<i32, Output = Self>
    + std::ops::Shl<i32, Output = Self>
    + std::ops::ShrAssign<i32>
{
}

impl ModOps for u8 {}
impl ModOps for u16 {}
impl ModOps for u32 {}
impl ModOps for u64 {}
impl ModOps for u128 {}
impl ModOps for usize {}
impl ModOps for i8 {}
impl ModOps for i16 {}
impl ModOps for i32 {}
impl ModOps for i64 {}
impl ModOps for i128 {}
impl ModOps for isize {}

pub fn mul_mod<T>(mut a: T, mut b: T, m: T) -> T
where
    T: ModOps,
{
    let mut product = T::zero();
    a %= m;
    while b > T::ZERO {
        if b & T::ONE == T::ONE {
            product = (a + product) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    product
}

pub fn div_mod<T>(a: T, b: T, m: T) -> T
where
    T: ModOps,
{
    let mut u = b;
    let mut v = m;
    let (mut x1, mut x2) = (T::ONE, T::ZERO);
    while u != T::ONE {
        let q = v / u;
        let r = v - q * u;
        let x = sub_mod(x2, x1 * q, m);
        v = u;
        u = r;
        x2 = x1;
        x1 = x;
    }
    mul_mod(a, x1, m)
}

pub fn add_mod<T>(a: T, b: T, m: T) -> T
where
    T: ModOps,
{
    ((a % m) + (b % m)) % m
}

pub fn sub_mod<T>(a: T, b: T, m: T) -> T
where
    T: ModOps,
{
    add_mod(a, m - (b % m), m)
}

pub fn pow_mod<T>(a: T, e: T, m: T) -> T
where
    T: ModOps,
{
    if e == T::zero() {
        return T::one();
    }
    if e == T::one() {
        return a % m;
    }
    if e & T::one() == T::one() {
        return mul_mod(a, pow_mod(a, e - T::one(), m), m);
    }
    let p = pow_mod(a, e >> 1, m);
    mul_mod(p, p, m)
}

pub trait Modular<T>:
    Sized
    + Copy
    + Clone
    + Send
    + Sync
    + From<T>
    + num_traits::One
    + num_traits::ConstOne
    + num_traits::Zero
    + num_traits::ConstZero
    + num_traits::Pow<T>
    + std::ops::Neg
    + std::ops::Add
    + std::ops::AddAssign
    + std::ops::Add<T>
    + std::ops::AddAssign<T>
    + for<'a> std::ops::Add<&'a Self>
    + for<'a> std::ops::AddAssign<&'a Self>
    + for<'a> std::ops::Add<&'a T>
    + for<'a> std::ops::AddAssign<&'a T>
    + std::ops::Sub
    + std::ops::SubAssign
    + std::ops::Sub<T>
    + std::ops::SubAssign<T>
    + for<'a> std::ops::Sub<&'a Self>
    + for<'a> std::ops::SubAssign<&'a Self>
    + for<'a> std::ops::Sub<&'a T>
    + for<'a> std::ops::SubAssign<&'a T>
    + std::ops::Mul
    + std::ops::MulAssign
    + std::ops::Mul<T>
    + std::ops::MulAssign<T>
    + for<'a> std::ops::Mul<&'a Self>
    + for<'a> std::ops::MulAssign<&'a Self>
    + for<'a> std::ops::Mul<&'a T>
    + for<'a> std::ops::MulAssign<&'a T>
    + std::ops::Div
    + std::ops::DivAssign
    + std::ops::Div<T>
    + std::ops::DivAssign<T>
    + for<'a> std::ops::Div<&'a Self>
    + for<'a> std::ops::DivAssign<&'a Self>
    + for<'a> std::ops::Div<&'a T>
    + for<'a> std::ops::DivAssign<&'a T>
    + std::iter::Sum
    + std::iter::Product
{
    const MOD: T;
    const TEN: Self;

    fn value(&self) -> T;

    fn inverse(&self) -> Self;

    fn inverse_all(values: &[Self]) -> Vec<Self>;
}

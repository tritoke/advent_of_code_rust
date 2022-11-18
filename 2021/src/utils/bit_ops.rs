pub trait BitOps {
    fn set_bit(&mut self, n: u32);
    fn test_bit(&self, n: u32) -> bool;
    fn clear_bit(&mut self, n: u32);
}

macro_rules! impl_bitops {
    ($t:ty) => {
        impl BitOps for $t {
            fn set_bit(&mut self, n: u32) {
                *self |= 1 << n;
            }

            fn test_bit(&self, n: u32) -> bool {
                (*self & (1 << n)) == (1 << n)
            }

            fn clear_bit(&mut self, n: u32) {
                *self &= !(1 << n);
            }
        }
    };

    ($t:ty, $($ts:ty),+) => {
        impl_bitops! { $t }
        impl_bitops! { $($ts),+ }
    };
}

impl_bitops! { usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8 }

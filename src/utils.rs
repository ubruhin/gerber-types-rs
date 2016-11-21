//! Some general utils used in other modules

//! Some general utils used in other modules

/// Integer division macro with proper rounding to the nearest number
macro_rules! rounded_div_macro {
    ($class:ty) => {
        pub fn rounded_div<$class>(dividend: $class, divisor: $class) -> $class {
            if dividend.is_positive() == divisor.is_positive() {
                (dividend + (divisor / 2)) / divisor
            } else {
                (dividend - (divisor / 2)) / divisor
            }
        }
    }
}

/// Integer division functions with proper rounding to the nearest number
rounded_div_macro!(i8);
rounded_div_macro!(i16);
rounded_div_macro!(i32);
rounded_div_macro!(i64);


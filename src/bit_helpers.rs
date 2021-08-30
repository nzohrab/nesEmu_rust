#[macro_export]
macro_rules! count_items {
    ($name:expr) => { 1 };
    ($first:expr, $($rest:expr),*) => {
        1 + crate::count_items!($($rest),*)
    }
}


pub fn pow_16(a: u16, b: u16) -> u16{
    let mut out = 1;
    for i in 0..b{
        out *= a
    }
    return out;
}
pub fn pow_8(a: u8, b: u8) -> u8{
    let mut out = 1;
    for i in 0..b{
        out *= a
    }
    return out;
}


#[macro_export]
macro_rules! bit_sum_16 {
    ($name:expr) => { 
        ($name as u16) * 1
    };
    ($first:expr, $($rest:expr),*) => {
        ($first as u16) * $crate::bit_helpers::pow_16(2,crate::count_items!($($rest),*)) + bit_sum_16!($($rest),*)
    }
}

#[macro_export]
macro_rules! bit_sum_8 {
    ($name:expr) => { 
        ($name as u8) * 1
    };
    ($first:expr, $($rest:expr),*) => {
        ($first as u8) * $crate::bit_helpers::pow_8(2,crate::count_items!($($rest),*)) + bit_sum_8!($($rest),*)
    }
}
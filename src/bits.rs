use std::collections::HashMap;
use std::ops::Range;

#[macro_export]
macro_rules! my_bits{
    ($( $key: expr => $val: expr),*) => {{
        let mut map: ::std::collections::HashMap<String, std::ops::Range<usize>> = ::std::collections::HashMap::new();
        $( map.insert($key.into(), $val); )*
        BitMapper{
            bits: 0u128,
            mappy: map,
        }
    }}
}


pub struct BitMapper{
    pub bits: u128,
    pub mappy: HashMap<String, Range<usize>>
}

impl BitMapper{
    pub fn get(&self, bits_id: &str) -> u128{
        assert_eq!(self.mappy.contains_key(bits_id), true, "{} not defined", bits_id);
        let range = &self.mappy[bits_id];

        let mask = (1 << (range.end - range.start)) - 1;
        return (self.bits >> range.start) & mask;                
    }

    pub fn set(&mut self, bits_id: &str, value: u128){
        assert_eq!(self.mappy.contains_key(bits_id), true, "{} not defined", bits_id);
        let range = &self.mappy[bits_id];
        for i in range.start..range.end{
            let x = 1u128&(value >> i-range.start);
            self.bits = (self.bits & !(1 << i)) | (x << i);
        }
    }
}
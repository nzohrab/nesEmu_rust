use fixed_map::{Key, Map};
use std::ops::Range;

#[macro_export]
macro_rules! my_bits{
    ($( $key: expr => $val: expr),*) => {{
        let mut map: ::fixed_map::Map<BitsId, std::ops::Range<usize>> = ::fixed_map::Map::new();
        $( map.insert($key, $val); )*
        BitMapper{
            bits: 0u16,
            mappy: map,
        }
    }}
}

#[derive(Clone, Copy, Key, Debug)]
pub enum BitsId{
    Unused,
    SpriteOverflow,
    SpriteZeroHit,
    VerticalBlank,
    Grayscale,
    RenderBackgroundLeft,
    RenderSpritesLeft,
    RenderBackground,
    RenderSprites,
    EnhanceRed,
    EnhanceBlue,
    EnhanceGreen,
    NametableX,
    NametableY,
    IncrementMode,
    PatternSprite,
    PatternBackground,
    SpriteSize,
    SlaveMode,
    EnableNmi,
    CoarseX,
    CoarseY,
    FineY
}

pub struct BitMapper{
    pub bits: u16,
    pub mappy: Map<BitsId, Range<usize>>
}

impl BitMapper{
    pub fn get(&self, bits_id: BitsId) -> u16{
        debug_assert!(self.mappy.get(bits_id).is_some(), "{:?} not defined", bits_id);
        let range = &self.mappy.get(bits_id).unwrap();

        let mask = (1 << (range.end - range.start)) - 1;
        return (self.bits >> range.start) & mask;                
    }

    pub fn set(&mut self, bits_id: BitsId, value: u16){
        debug_assert!(self.mappy.get(bits_id).is_some(), "{:?} not defined", bits_id);
        let range = &self.mappy.get(bits_id).unwrap();
        for i in range.start..range.end{
            let x = 1u16&(value >> i-range.start);
            self.bits = (self.bits & !(1 << i)) | (x << i);
        }
    }
}

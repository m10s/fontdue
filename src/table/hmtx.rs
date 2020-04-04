use crate::parse::*;
use crate::FontResult;
use alloc::vec::*;

// Apple: https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6hmtx.html
// Microsoft: https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HMetric {
    pub advance_width: u16,
    pub left_side_bearing: i16,
}

#[derive(Debug, PartialEq)]
pub struct TableHmtx {
    /// Indexed by glyph id.
    pub hmetrics: Vec<HMetric>,
}

impl TableHmtx {
    pub fn new(hmtx: &[u8], num_glyphs: u16, num_long_hmetrics: u16) -> FontResult<TableHmtx> {
        let mut stream = Stream::new(hmtx);
        let mut hmetrics = Vec::with_capacity(num_glyphs as usize);
        let mut advance_width = 0;
        for _ in 0..num_long_hmetrics {
            advance_width = stream.read_u16();
            let left_side_bearing = stream.read_i16();
            hmetrics.push(HMetric {
                advance_width,
                left_side_bearing,
            });
        }
        for _ in 0..(num_glyphs - num_long_hmetrics) {
            let left_side_bearing = stream.read_i16();
            hmetrics.push(HMetric {
                advance_width,
                left_side_bearing,
            });
        }
        Ok(TableHmtx {
            hmetrics,
        })
    }
}

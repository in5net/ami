use std::ops::Range;

pub struct AmiError {
    pub msg: String,
    pub reason: String,
    pub range: Range<usize>,
}

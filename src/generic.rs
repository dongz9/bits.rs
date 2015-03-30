/// Logical and not
#[inline(always)]
pub fn and_not(src1: u64, src2: u64) -> u64 {
    !src1 & src2
}

/// Gather bits
#[inline(always)]
pub fn gather(val: u64, mut mask: u64) -> u64 {
    let mut res = 0u64;
    let mut bb  = 1u64;
    while  mask                 != 0 {
        if  val &  i_get (mask) != 0 {
            res |= bb;
        }
           mask  = i_flip(mask);
             bb += bb;
    }
    res
}

/// Get bit block
#[inline(always)]
pub fn block(src: u64, start: u32, len: u32) -> u64 {
    (src >> start) & ((1 << len) - 1)
}

/// Scatter bits
#[inline(always)]
pub fn scatter(val: u64, mut mask: u64) -> u64 {
    let mut res = 0u64;
    let mut bb  = 1u64;
    while  mask       != 0 {
        if  val &  bb != 0 {
            res |= i_get (mask);
        }
           mask  = i_flip(mask);
             bb += bb;
    }
    res
}

/// Zero high bits from specified position
#[inline(always)]
pub fn zero_high_from_pos(src: u64, index: u32) -> u64 {
    src & ((1 << index) - 1)
}

/// Count total 1s; population count
#[inline(always)]
pub fn i_count_total(src: u64) -> u64 { unsafe {
    ::std::intrinsics::ctpop64(src)
}}

/// Fill up to lowest 1
#[inline(always)]
pub fn i_fill_upto(src: u64) -> u64 {
    src | src.wrapping_sub(1)
}

/// Get lowest 1
#[allow(unsigned_negation)]
#[inline(always)]
pub fn i_get(src: u64) -> u64 {
    src & -src
}

/// Get lowest 1 and complement word
#[inline(always)]
pub fn i_get_invert(src: u64) -> u64 {
    !src | src.wrapping_sub(1)
}

/// Mask trailing 1s and complement word
#[inline(always)]
pub fn i_mask_upto_invert(src: u64) -> u64 {
    !src | (src + 1)
}

/// Mask up to lowest 1
#[inline(always)]
pub fn i_mask_upto(src: u64) -> u64 {
    src ^ src.wrapping_sub(1)
}

/// Set lowest 1 to 0
#[inline(always)]
pub fn i_flip(src: u64) -> u64 {
    src & src.wrapping_sub(1)
}

/// Count leading 0s
#[inline(always)]
pub fn o_count_leading(src: u64) -> u64 { unsafe {
    ::std::intrinsics::ctlz64(src)
}}

/// Count trailing 0s
#[inline(always)]
pub fn o_count_trailing(src: u64) -> u64 { unsafe {
    ::std::intrinsics::cttz64(src)
}}

/// Set lowest 0 to 1
#[inline(always)]
pub fn o_flip(src: u64) -> u64 {
    src | (src + 1)
}

/// Fill up to lowest 0
#[inline(always)]
pub fn o_fill_upto(src: u64) -> u64 {
    src & (src + 1)
}

/// Get lowest 0
#[inline(always)]
pub fn o_get(src: u64) -> u64 {
    src | !(src + 1)
}

/// Get lowest 0 and complement word
#[inline(always)]
pub fn o_get_invert(src: u64) -> u64 {
    !src & (src + 1)
}

/// Mask up to lowest 0
#[inline(always)]
pub fn o_mask_upto(src: u64) -> u64 {
    src ^ (src + 1)
}

/// Mask trailing 0s
#[inline(always)]
pub fn o_mask_upto_invert(src: u64) -> u64 {
    !src & src.wrapping_sub(1)
}

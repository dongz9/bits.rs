/// Logical and not
#[inline(always)]
pub fn b_and_not(src1: u64, src2: u64) -> u64 {
    !src1 & src2
}

/// Get bit block
#[inline(always)]
pub fn b_get_block_from(src: u64, start: u32, len: u32) -> u64 {
    (src >> start) & ((1 << len) - 1)
}

/// Get bits included in blocks k > 1 count bits of the same parity
#[inline(always)]
pub fn b_get_block(src: u64) -> u64 {
    !b_get_singletons(src)
}

/// Gather bits
#[inline(always)]
pub fn b_gather(val: u64, mut mask: u64) -> u64 {
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

/// Return 0s and 1s on block transitions within outer 0s
#[inline(always)]
pub fn b_get_border(src: u64) -> u64 {
    (src ^ (src << 1)) & (src ^ (src >> 1))
}

/// Get bits bounded on the left and right by bits of the same parity
#[inline(always)]
pub fn b_get_interior(src: u64) -> u64 {
    !b_get_border(src)
}

/// Get singleton 0s and 1s within outer 0s
#[inline(always)]
pub fn b_get_singletons(src: u64) -> u64 {
    (src ^ (src << 1)) & (src ^ (src >> 1))
}

/// Get singleton 0s and 1s
#[inline(always)]
pub fn b_get_singletons_xi(src: u64) -> u64 {
    i_get_singletons_xi(src) | o_get_singletons_xi(src)
}

/// Scatter bits
#[inline(always)]
pub fn b_scatter(val: u64, mut mask: u64) -> u64 {
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
pub fn b_zero_high_from_pos(src: u64, index: u32) -> u64 {
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

/// Get 1s included in blocks k > 1 count 1s
#[inline(always)]
pub fn i_get_block(src: u64) -> u64 {
    src & ((src << 1) | (src >> 1))
}

/// Get 1s bordering blocks of k > 1 count 1s
#[inline(always)]
pub fn i_get_block_border(src: u64) -> u64 {
    src & ((src << 1) ^ (src >> 1))
}

/// Get 1s to the left of blocks of k > 1 count 1s
#[inline(always)]
pub fn i_get_block_border_low(src: u64) -> u64 {
    let src_shr = src >> 1;
    (src & ((src << 1) ^ src_shr)) & src_shr
}

/// Get 1s to the right of blocks of k > 1 count 1s
#[inline(always)]
pub fn i_get_block_border_high(src: u64) -> u64 {
    let src_shl = src << 1;
    (src & (src_shl ^ (src >> 1))) & src_shl
}

/// Get 1s bordering 0s
#[inline(always)]
pub fn i_get_border(src: u64) -> u64 {
    src & !((src << 1) & (src >> 1))
}

/// Get 1s to the right of 0s
#[inline(always)]
pub fn i_get_border_high(src: u64) -> u64 {
    src & (src ^ (src >> 1))
}

/// Get 1s to the left of 0s
#[inline(always)]
pub fn i_get_border_low(src: u64) -> u64 {
    src & (src ^ (src << 1))
}

/// Get 1s bounded by 1s on the left and right
#[inline(always)]
pub fn i_get_interior(src: u64) -> u64 {
    src & ((src << 1) & (src >> 1))
}

/// Get lowest 1 and complement word
#[inline(always)]
pub fn i_get_invert(src: u64) -> u64 {
    !src | src.wrapping_sub(1)
}

/// Get singleton 1s
#[inline(always)]
pub fn i_get_singletons(src: u64) -> u64 {
    src & !((src << 1) | (src >> 1))
}

/// Get singleton 1s within outer 0s
#[inline(always)]
pub fn i_get_singletons_xi(src: u64) -> u64 {
    i_get_singletons(src)
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

/// Get singleton 0s
#[inline(always)]
pub fn o_get_singletons(src: u64) -> u64 {
    !src & ((src << 1) & (src >> 1))
}

/// Get singleton 0s within outer 0s
#[inline(always)]
pub fn o_get_singletons_xi(src: u64) -> u64 {
    i_get_singletons(!src)
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

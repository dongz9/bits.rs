use std::{num, ops};

pub trait BitOps: num::Int {
    /// Logical and not
    #[inline(always)]
    fn b_and_not(self, rhs: Self) -> Self {
        !self & rhs
    }

    /// Get bit block
    #[inline(always)]
    fn b_get_block_from(self, start: usize, len: usize) -> Self {
        (self >> start) & ((Self::one() << len) - Self::one())
    }

    /// Get bits included in blocks k > 1 count bits of the same parity
    #[inline(always)]
    fn b_get_block(self) -> Self {
        !self.b_get_singletons()
    }

    /// Gather bits
    #[inline(always)]
    fn b_gather(self, mut mask: Self) -> Self where Self: ops::Neg<Output = Self> {
        let mut res = Self::zero();
        let mut bin = Self::one ();
        while  mask                 != Self::zero() {
            if self & mask.i_get () != Self::zero() {
                res = res | bin;
            }
               mask = mask.i_flip();
                bin = bin + bin;
        }
        res
    }

    /// Return 0s and 1s on block transitions within outer 0s
    #[inline(always)]
    fn b_get_border(self) -> Self {
        (self ^ (self << 1)) | (self ^ (self >> 1))
    }

    /// Get bits bounded on the left and right by bits of the same parity
    #[inline(always)]
    fn b_get_interior(self) -> Self {
        !self.b_get_border()
    }

    /// Get singleton 0s and 1s within outer 0s
    #[inline(always)]
    fn b_get_singletons(self) -> Self {
        (self ^ (self << 1)) & (self ^ (self >> 1))
    }

    /// Get singleton 0s and 1s
    #[inline(always)]
    fn b_get_singletons_xi(self) -> Self {
        self.i_get_singletons_xi() | self.o_get_singletons_xi()
    }

    /// Scatter bits
    #[inline(always)]
    fn b_scatter(self, mut mask: Self) -> Self where Self: ops::Neg<Output = Self> {
        let mut res = Self::zero();
        let mut bin = Self::one ();
        while  mask       != Self::zero() {
            if self & bin != Self::zero() {
                res = res | mask.i_get ();
            }
               mask = mask.i_flip();
                bin = bin + bin;
        }
        res
    }

    /// Zero high bits from specified position
    #[inline(always)]
    fn b_zero_high_from_pos(self, index: usize) -> Self {
        self & ((Self::one() << index) - Self::one())
    }

    /// Count total 1s; population count
    #[inline(always)]
    fn i_count_total(self) -> u32 {
        self.count_ones()
    }

    /// Fill up to lowest 1
    #[inline(always)]
    fn i_fill_upto(self) -> Self {
        self | self.wrapping_sub(Self::one())
    }

    /// Get lowest 1
    #[allow(unsigned_negation)]
    #[inline(always)]
    fn i_get(self) -> Self where Self: ops::Neg<Output = Self> {
        self & -self
    }

    /// Get 1s included in blocks k > 1 count 1s
    #[inline(always)]
    fn i_get_block(self) -> Self {
        self & ((self << 1) | (self >> 1))
    }

    /// Get 1s bordering blocks of k > 1 count 1s
    #[inline(always)]
    fn i_get_block_border(self) -> Self {
        self & ((self << 1) ^ (self >> 1))
    }

    /// Get 1s to the left of blocks of k > 1 count 1s
    #[inline(always)]
    fn i_get_block_border_low(self) -> Self {
        let shr = self >> 1;
        (self & ((self << 1) ^ shr)) & shr
    }

    /// Get 1s to the right of blocks of k > 1 count 1s
    #[inline(always)]
    fn i_get_block_border_high(self) -> Self {
        let shl = self << 1;
        (self & (shl ^ (self >> 1))) & shl
    }

    /// Get 1s bordering 0s
    #[inline(always)]
    fn i_get_border(self) -> Self {
        self & !((self << 1) & (self >> 1))
    }

    /// Get 1s to the right of 0s
    #[inline(always)]
    fn i_get_border_high(self) -> Self {
        self & (self ^ (self >> 1))
    }

    /// Get 1s to the left of 0s
    #[inline(always)]
    fn i_get_border_low(self) -> Self {
        self & (self ^ (self << 1))
    }

    /// Get 1s bounded by 1s on the left and right
    #[inline(always)]
    fn i_get_interior(self) -> Self {
        self & ((self << 1) & (self >> 1))
    }

    /// Get lowest 1 and complement word
    #[inline(always)]
    fn i_get_invert(self) -> Self {
        !self | self.wrapping_sub(Self::one())
    }

    /// Get singleton 1s within outer 0s
    #[inline(always)]
    fn i_get_singletons(self) -> Self {
        self & !((self << 1) | (self >> 1))
    }

    /// Get singleton 1s
    #[inline(always)]
    fn i_get_singletons_xi(self) -> Self {
        self.i_get_singletons()
    }

    /// Mask trailing 1s and complement word
    #[inline(always)]
    fn i_mask_upto_invert(self) -> Self {
        !self | (self + Self::one())
    }

    /// Mask up to lowest 1
    #[inline(always)]
    fn i_mask_upto(self) -> Self {
        self ^ self.wrapping_sub(Self::one())
    }

    /// Set lowest 1 to 0
    #[inline(always)]
    fn i_flip(self) -> Self {
        self & self.wrapping_sub(Self::one())
    }

    /// Count leading 0s
    #[inline(always)]
    fn o_count_leading(self) -> u32 {
        self.leading_zeros()
    }

    /// Count total 0s
    #[inline(always)]
    fn o_count_total(self) -> u32 {
        self.count_zeros()
    }

    /// Count trailing 0s
    #[inline(always)]
    fn o_count_trailing(self) -> u32 {
        self.trailing_zeros()
    }

    /// Set lowest 0 to 1
    #[inline(always)]
    fn o_flip(self) -> Self {
        self | (self + Self::one())
    }

    /// Fill up to lowest 0
    #[inline(always)]
    fn o_fill_upto(self) -> Self {
        self & (self + Self::one())
    }

    /// Get lowest 0
    #[inline(always)]
    fn o_get(self) -> Self {
        self | !(self + Self::one())
    }

    /// Get lowest 0 and complement word
    #[inline(always)]
    fn o_get_invert(self) -> Self {
        !self & (self + Self::one())
    }

    /// Get singleton 0s within outer 0s
    #[inline(always)]
    fn o_get_singletons(self) -> Self {
        !self & ((self << 1) & (self >> 1))
    }

    /// Get singleton 0s
    #[inline(always)]
    fn o_get_singletons_xi(self) -> Self {
        (!self).i_get_singletons()
    }

    /// Mask up to lowest 0
    #[inline(always)]
    fn o_mask_upto(self) -> Self {
        self ^ (self + Self::one())
    }

    /// Mask trailing 0s
    #[inline(always)]
    fn o_mask_upto_invert(self) -> Self {
        !self & self.wrapping_sub(Self::one())
    }
}

impl<T: num::Int> BitOps for T {}

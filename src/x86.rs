/// ABM: Advanced Bit Manipulation Instruction Set
pub mod abm {
    pub mod u64 {
        use generic;

        /// Count leading 0s
        #[inline(always)]
        pub fn lzcnt(src: u64) -> u64 {
            generic::o_count_leading(src)
        }

        /// Count total 1s; population count
        #[inline(always)]
        pub fn popcnt(src: u64) -> u64 {
            generic::i_count_total(src)
        }
    }
}

/// BMI1: Bit Manipulation Instruction Set #1
pub mod bmi1 {
    pub mod u64 {
        use generic;

        /// Logical and not
        #[inline(always)]
        pub fn andn(src1: u64, src2: u64) -> u64 {
            generic::b_and_not(src1, src2)
        }

        /// Get bit block
        #[inline(always)]
        pub fn bextr(src: u64, start: u32, len: u32) -> u64 {
            generic::b_get_block_from(src, start, len)
        }

        /// Get lowest 1
        #[allow(unsigned_negation)]
        #[inline(always)]
        pub fn blsi(src: u64) -> u64 {
            generic::i_get(src)
        }

        /// Mask up to lowest 1
        #[inline(always)]
        pub fn blsmsk(src: u64) -> u64 {
            generic::i_mask_upto(src)
        }

        /// Set lowest 1 to 0
        #[inline(always)]
        pub fn blsr(src: u64) -> u64 {
            generic::i_flip(src)
        }

        /// Count trailing 0s
        #[inline(always)]
        pub fn tzcnt(src: u64) -> u64 {
            generic::o_count_trailing(src)
        }
    }
}

/// BMI2: Bit Manipulation Instruction Set #2
pub mod bmi2 {
    pub mod u64 {
        use generic;

        /// Zero high bits from specified position
        #[inline(always)]
        pub fn bzhi(src: u64, index: u32) -> u64 {
            generic::b_zero_high_from_pos(src, index)
        }

        /// Parallel bits deposit; scatter bits
        #[inline(always)]
        pub fn pdep(val: u64, mask: u64) -> u64 {
            generic::b_scatter(val, mask)
        }

        /// Parallel bits extract; gather bits
        #[inline(always)]
        pub fn pext(val: u64, mask: u64) -> u64 {
            generic::b_gather(val, mask)
        }
    }
}

/// TBM: Trailing Bit Manipulation Instruction Set
pub mod tbm {
    pub mod u64 {
        use generic;

        /// Fill up to lowest 0
        #[inline(always)]
        pub fn blcfill(src: u64) -> u64 {
            generic::o_fill_upto(src)
        }

        /// Get lowest 0
        #[inline(always)]
        pub fn blci(src: u64) -> u64 {
            generic::o_get(src)
        }

        /// Get lowest 0 and complement word
        #[inline(always)]
        pub fn blcic(src: u64) -> u64 {
            generic::o_get_invert(src)
        }

        /// Mask up to lowest 0
        #[inline(always)]
        pub fn blcmsk(src: u64) -> u64 {
            generic::o_mask_upto(src)
        }

        /// Set lowest 0 to 1
        #[inline(always)]
        pub fn blcs(src: u64) -> u64 {
            generic::o_flip(src)
        }

        /// Fill up to lowest 1
        #[inline(always)]
        pub fn blsfill(src: u64) -> u64 {
            generic::i_fill_upto(src)
        }

        /// Get lowest 1 and complement word
        #[inline(always)]
        pub fn blsic(src: u64) -> u64 {
            generic::i_get_invert(src)
        }

        /// Mask trailing 1s and complement word
        #[inline(always)]
        pub fn t1mskc(src: u64) -> u64 {
            generic::i_mask_upto_invert(src)
        }

        /// Mask trailing 0s
        #[inline(always)]
        pub fn tzmsk(src: u64) -> u64 {
            generic::o_mask_upto_invert(src)
        }
    }
}


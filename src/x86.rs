/// ABM: Advanced Bit Manipulation Instruction Set
pub mod abm {
    pub mod u64 {
        use generic::BitOps;

        /// Count leading 0s
        #[inline(always)]
        pub fn lzcnt(src: u64) -> u64 {
            src.o_count_leading() as u64
        }

        /// Count total 1s; population count
        #[inline(always)]
        pub fn popcnt(src: u64) -> u64 {
            src.i_count_total() as u64
        }
    }
}

/// BMI1: Bit Manipulation Instruction Set #1
pub mod bmi1 {
    pub mod u64 {
        use generic::BitOps;

        /// Logical and not
        #[inline(always)]
        pub fn andn(src1: u64, src2: u64) -> u64 {
            src1.b_and_not(src2)
        }

        /// Get bit block
        #[inline(always)]
        pub fn bextr(src: u64, start: u32, len: u32) -> u64 {
            src.b_get_block_from(start as usize, len as usize)
        }

        /// Get lowest 1
        #[allow(unsigned_negation)]
        #[inline(always)]
        pub fn blsi(src: u64) -> u64 {
            src.i_get()
        }

        /// Mask up to lowest 1
        #[inline(always)]
        pub fn blsmsk(src: u64) -> u64 {
            src.i_mask_upto()
        }

        /// Set lowest 1 to 0
        #[inline(always)]
        pub fn blsr(src: u64) -> u64 {
            src.i_flip()
        }

        /// Count trailing 0s
        #[inline(always)]
        pub fn tzcnt(src: u64) -> u64 {
            src.o_count_trailing() as u64
        }
    }
}

/// BMI2: Bit Manipulation Instruction Set #2
pub mod bmi2 {
    pub mod u64 {
        use generic::BitOps;

        /// Zero high bits from specified position
        #[inline(always)]
        pub fn bzhi(src: u64, index: u32) -> u64 {
            src.b_zero_high_from_pos(index as usize)
        }

        /// Parallel bits deposit; scatter bits
        #[inline(always)]
        pub fn pdep(val: u64, mask: u64) -> u64 {
            val.b_scatter(mask)
        }

        /// Parallel bits extract; gather bits
        #[inline(always)]
        pub fn pext(val: u64, mask: u64) -> u64 {
            val.b_gather(mask)
        }
    }
}

/// TBM: Trailing Bit Manipulation Instruction Set
pub mod tbm {
    pub mod u64 {
        use generic::BitOps;

        /// Fill up to lowest 0
        #[inline(always)]
        pub fn blcfill(src: u64) -> u64 {
            src.o_fill_upto()
        }

        /// Get lowest 0
        #[inline(always)]
        pub fn blci(src: u64) -> u64 {
            src.o_get()
        }

        /// Get lowest 0 and complement word
        #[inline(always)]
        pub fn blcic(src: u64) -> u64 {
            src.o_get_invert()
        }

        /// Mask up to lowest 0
        #[inline(always)]
        pub fn blcmsk(src: u64) -> u64 {
            src.o_mask_upto()
        }

        /// Set lowest 0 to 1
        #[inline(always)]
        pub fn blcs(src: u64) -> u64 {
            src.o_flip()
        }

        /// Fill up to lowest 1
        #[inline(always)]
        pub fn blsfill(src: u64) -> u64 {
            src.i_fill_upto()
        }

        /// Get lowest 1 and complement word
        #[inline(always)]
        pub fn blsic(src: u64) -> u64 {
            src.i_get_invert()
        }

        /// Mask trailing 1s and complement word
        #[inline(always)]
        pub fn t1mskc(src: u64) -> u64 {
            src.i_mask_upto_invert()
        }

        /// Mask trailing 0s
        #[inline(always)]
        pub fn tzmsk(src: u64) -> u64 {
            src.o_mask_upto_invert()
        }
    }
}

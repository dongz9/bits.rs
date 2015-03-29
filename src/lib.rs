#![feature(core)]

pub mod u64 {
    // ABM /////////////////////////////////////////////////////////////////////

    /// Count leading zero bits
    #[inline(always)]
    pub fn lzcnt(src: u64) -> u64 { unsafe {
        ::std::intrinsics::ctlz64(src)
    }}

    /// Population count
    #[inline(always)]
    pub fn popcnt(src: u64) -> u64 { unsafe {
        ::std::intrinsics::ctpop64(src)
    }}

    // BMI1 ////////////////////////////////////////////////////////////////////

    /// Logical and not
    #[inline(always)]
    pub fn andn(src1: u64, src2: u64) -> u64 {
        !src1 & src2
    }

    /// Bit field extract
    #[inline(always)]
    pub fn bextr(src: u64, start: u32, len: u32) -> u64 {
        (src >> start) & ((1 << len) - 1)
    }

    /// Extract lowest set isolated bit
    #[allow(unsigned_negation)]
    #[inline(always)]
    pub fn blsi(src: u64) -> u64 {
        src & -src
    }

    /// Get mask up to lowest set bit
    #[inline(always)]
    pub fn blsmsk(src: u64) -> u64 {
        src ^ src.wrapping_sub(1)
    }

    /// Reset lowest set bit
    #[inline(always)]
    pub fn blsr(src: u64) -> u64 {
        src & src.wrapping_sub(1)
    }

    /// Count trailing zero bits
    #[inline(always)]
    pub fn tzcnt(src: u64) -> u64 { unsafe {
        ::std::intrinsics::cttz64(src)
    }}

    // BMI2 ////////////////////////////////////////////////////////////////////

    /// Zero high bits from specified position
    #[inline(always)]
    pub fn bzhi(src: u64, index: u32) -> u64 {
        src & ((1 << index) - 1)
    }

    /// Parallel bits deposit
    #[inline(always)]
    pub fn pdep(val: u64, mut mask: u64) -> u64 {
        let mut res = 0u64;
        let mut bb  = 1u64;
        while  mask       != 0 {
            if  val &  bb != 0 {
                res |= blsi(mask);
            }
               mask  = blsr(mask);
                 bb += bb;
        }
        res
    }

    /// Parallel bits extract
    #[inline(always)]
    pub fn pext(val: u64, mut mask: u64) -> u64 {
        let mut res = 0u64;
        let mut bb  = 1u64;
        while  mask               != 0 {
            if  val &  blsi(mask) != 0 {
                res |= bb;
            }
               mask  = blsr(mask);
                 bb += bb;
        }
        res
    }

    // TBM /////////////////////////////////////////////////////////////////////

    /// Fill from lowest clear bit
    #[inline(always)]
    pub fn blcfill(src: u64) -> u64 {
        src & (src + 1)
    }

    /// Isolate lowest clear bit
    #[inline(always)]
    pub fn blci(src: u64) -> u64 {
        src | !(src + 1)
    }

    /// Isolate lowest clear bit and complement
    #[inline(always)]
    pub fn blcic(src: u64) -> u64 {
        !src & (src + 1)
    }

    /// Mask from lowest clear bit
    #[inline(always)]
    pub fn blcmsk(src: u64) -> u64 {
        src ^ (src + 1)
    }

    /// Set lowest clear bit
    #[inline(always)]
    pub fn blcs(src: u64) -> u64 {
        src | (src + 1)
    }

    /// Fill from lowest set bit
    #[inline(always)]
    pub fn blsfill(src: u64) -> u64 {
        src | src.wrapping_sub(1)
    }

    /// Isolate lowest set bit and complement
    #[inline(always)]
    pub fn blsic(src: u64) -> u64 {
        !src | src.wrapping_sub(1)
    }

    /// Inverse mask from trailing ones
    #[inline(always)]
    pub fn t1mskc(src: u64) -> u64 {
        !src | (src + 1)
    }

    /// Mask from trailing zeros
    #[inline(always)]
    pub fn tzmsk(src: u64) -> u64 {
        !src & src.wrapping_sub(1)
    }
}

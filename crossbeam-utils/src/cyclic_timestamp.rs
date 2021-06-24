use timestamp::Timestamp;

/// A cyclic timestamp value.  For a numeric type having 2^n bits, a
/// CyclicTimestamp guarantees that any timestamp is less than its
/// next 2^k - 1 successors, where k = n - 2.
///
/// For large numeric (such as u32 or u64), this can be used as a
/// version timestamp which will tolerate up to 2^30 and 2^62
/// concurrent successor values respectively.
///
/// This is particularly useful for versioned pointers in lock-free
/// protocols, where the number of concurrent successors is determined
/// by the maximum time any thread can take to attempt an lock-free
/// transaction (which is largely a function of how long it might be
/// stalled).  These tolerance levels guarantee the integrity of the
/// timestamp counter with overwhelming probability for local
/// threading models (and even many distributed ones).
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct CyclicTimestamp<S: Sized>(S);

// These are implemented by splitting the number into two sections.
// The lower 2^n-2 bits are a traditional counter.  The top 2 bits are
// used to implement a "rock-paper-scissors" (RPS) counter.
//
// The RPS ordering looks like 0 < 1 < 2 < 0.  Thus, the RPS counter
// tolerates at most two concurrent timestamps.  A value of 3 is an
// invalid RPS counter value.
//
// We bump the RPS counter when the lower bit counter overflows,
// resulting in a tolerance of 2^n-2 concurrent values.

impl Timestamp for CyclicTimestamp<u8> {
    fn new() -> CyclicTimestamp<u8> {
        CyclicTimestamp(0)
    }

    fn earlier(&self, other: &CyclicTimestamp<u8>) -> bool {
        let self_number = 0x3f & self.0;
        let other_number = 0x3f & other.0;
        let self_rps = self.0 & 0xc0;
        let other_rps = other.0 & 0xc0;

        (self_rps == other_rps && self_number < other_number)
            || ((self_rps == 0x00) && (other_rps == 0x40))
            || ((self_rps == 0x40) && (other_rps == 0x80))
            || ((self_rps == 0x80) && (other_rps == 0x00))
    }

    fn later(&self, other: &CyclicTimestamp<u8>) -> bool {
        let self_number = 0x3f & self.0;
        let other_number = 0x3f & other.0;
        let self_rps = self.0 & 0xc0;
        let other_rps = other.0 & 0xc0;

        (self_rps == other_rps && self_number > other_number)
            || ((self_rps == 0x40) && (other_rps == 0x00))
            || ((self_rps == 0x80) && (other_rps == 0x40))
            || ((self_rps == 0x00) && (other_rps == 0x80))
    }

    fn succ(&self) -> CyclicTimestamp<u8> {
        if self.0 != 0xbf {
            CyclicTimestamp(self.0 + 1)
        } else {
            CyclicTimestamp(0)
        }
    }
}

impl Timestamp for CyclicTimestamp<u16> {
    fn new() -> CyclicTimestamp<u16> {
        CyclicTimestamp(0)
    }

    fn earlier(&self, other: &CyclicTimestamp<u16>) -> bool {
        let self_number = 0x3fff & self.0;
        let other_number = 0x3fff & other.0;
        let self_rps = self.0 & 0xc000;
        let other_rps = other.0 & 0xc000;

        (self_rps == other_rps && self_number < other_number)
            || ((self_rps == 0x0000) && (other_rps == 0x4000))
            || ((self_rps == 0x4000) && (other_rps == 0x8000))
            || ((self_rps == 0x8000) && (other_rps == 0x0000))
    }

    fn later(&self, other: &CyclicTimestamp<u16>) -> bool {
        let self_number = 0x3fff & self.0;
        let other_number = 0x3fff & other.0;
        let self_rps = self.0 & 0xc000;
        let other_rps = other.0 & 0xc000;

        (self_rps == other_rps && self_number > other_number)
            || ((self_rps == 0x4000) && (other_rps == 0x0000))
            || ((self_rps == 0x8000) && (other_rps == 0x4000))
            || ((self_rps == 0x0000) && (other_rps == 0x8000))
    }

    fn succ(&self) -> CyclicTimestamp<u16> {
        if self.0 != 0xbfff {
            CyclicTimestamp(self.0 + 1)
        } else {
            CyclicTimestamp(0)
        }
    }
}

impl Timestamp for CyclicTimestamp<u32> {
    fn new() -> CyclicTimestamp<u32> {
        CyclicTimestamp(0)
    }

    fn earlier(&self, other: &CyclicTimestamp<u32>) -> bool {
        let self_number = 0x3fffffff & self.0;
        let other_number = 0x3fffffff & other.0;
        let self_rps = self.0 & 0xc0000000;
        let other_rps = other.0 & 0xc0000000;

        (self_rps == other_rps && self_number < other_number)
            || ((self_rps == 0x00000000) && (other_rps == 0x40000000))
            || ((self_rps == 0x40000000) && (other_rps == 0x80000000))
            || ((self_rps == 0x80000000) && (other_rps == 0x00000000))
    }

    fn later(&self, other: &CyclicTimestamp<u32>) -> bool {
        let self_number = 0x3fffffff & self.0;
        let other_number = 0x3fffffff & other.0;
        let self_rps = self.0 & 0xc0000000;
        let other_rps = other.0 & 0xc0000000;

        (self_rps == other_rps && self_number > other_number)
            || ((self_rps == 0x40000000) && (other_rps == 0x00000000))
            || ((self_rps == 0x80000000) && (other_rps == 0x40000000))
            || ((self_rps == 0x00000000) && (other_rps == 0x80000000))
    }

    fn succ(&self) -> CyclicTimestamp<u32> {
        if self.0 != 0xbfffffff {
            CyclicTimestamp(self.0 + 1)
        } else {
            CyclicTimestamp(0)
        }
    }
}

impl Timestamp for CyclicTimestamp<u64> {
    fn new() -> CyclicTimestamp<u64> {
        CyclicTimestamp(0)
    }

    fn earlier(&self, other: &CyclicTimestamp<u64>) -> bool {
        let self_number = 0x3fffffffffffffff & self.0;
        let other_number = 0x3fffffffffffffff & other.0;
        let self_rps = self.0 & 0xc000000000000000;
        let other_rps = other.0 & 0xc000000000000000;

        (self_rps == other_rps && self_number < other_number)
            || ((self_rps == 0x0000000000000000) && (other_rps == 0x4000000000000000))
            || ((self_rps == 0x4000000000000000) && (other_rps == 0x8000000000000000))
            || ((self_rps == 0x8000000000000000) && (other_rps == 0x0000000000000000))
    }

    fn later(&self, other: &CyclicTimestamp<u64>) -> bool {
        let self_number = 0x3fffffffffffffff & self.0;
        let other_number = 0x3fffffffffffffff & other.0;
        let self_rps = self.0 & 0xc000000000000000;
        let other_rps = other.0 & 0xc000000000000000;

        (self_rps == other_rps && self_number > other_number)
            || ((self_rps == 0x4000000000000000) && (other_rps == 0x0000000000000000))
            || ((self_rps == 0x8000000000000000) && (other_rps == 0x4000000000000000))
            || ((self_rps == 0x0000000000000000) && (other_rps == 0x8000000000000000))
    }

    fn succ(&self) -> CyclicTimestamp<u64> {
        if self.0 != 0xbfffffffffffffff {
            CyclicTimestamp(self.0 + 1)
        } else {
            CyclicTimestamp(0)
        }
    }
}

impl Timestamp for CyclicTimestamp<u128> {
    fn new() -> CyclicTimestamp<u128> {
        CyclicTimestamp(0)
    }

    fn earlier(&self, other: &CyclicTimestamp<u128>) -> bool {
        let self_number = 0x3fffffffffffffffffffffffffffffff & self.0;
        let other_number = 0x3fffffffffffffffffffffffffffffff & other.0;
        let self_rps = self.0 & 0xc0000000000000000000000000000000;
        let other_rps = other.0 & 0xc0000000000000000000000000000000;

        (self_rps == other_rps && self_number < other_number)
            || ((self_rps == 0x00000000000000000000000000000000)
                && (other_rps == 0x40000000000000000000000000000000))
            || ((self_rps == 0x40000000000000000000000000000000)
                && (other_rps == 0x80000000000000000000000000000000))
            || ((self_rps == 0x80000000000000000000000000000000)
                && (other_rps == 0x00000000000000000000000000000000))
    }

    fn later(&self, other: &CyclicTimestamp<u128>) -> bool {
        let self_number = 0x3fffffffffffffffffffffffffffffff & self.0;
        let other_number = 0x3fffffffffffffffffffffffffffffff & other.0;
        let self_rps = self.0 & 0xc0000000000000000000000000000000;
        let other_rps = other.0 & 0xc0000000000000000000000000000000;

        (self_rps == other_rps && self_number > other_number)
            || ((self_rps == 0x40000000000000000000000000000000)
                && (other_rps == 0x00000000000000000000000000000000))
            || ((self_rps == 0x80000000000000000000000000000000)
                && (other_rps == 0x40000000000000000000000000000000))
            || ((self_rps == 0x00000000000000000000000000000000)
                && (other_rps == 0x80000000000000000000000000000000))
    }

    fn succ(&self) -> CyclicTimestamp<u128> {
        if self.0 != 0xbfffffffffffffffffffffffffffffff {
            CyclicTimestamp(self.0 + 1)
        } else {
            CyclicTimestamp(0)
        }
    }
}

#[test]
fn test_u8_wrap() {
    // Test that successor never gets into an invalid RPS value.  We
    // should never see anything with an RPS value of 0xc0.
    assert_eq!(CyclicTimestamp(0 as u8), CyclicTimestamp(0xbf as u8).succ())
}

#[test]
fn test_u16_wrap() {
    // Test that successor never gets into an invalid RPS value.  We
    // should never see anything with an RPS value of 0xc000.
    assert_eq!(
        CyclicTimestamp(0 as u16),
        CyclicTimestamp(0xbfff as u16).succ()
    )
}

#[test]
fn test_u32_wrap() {
    // Test that successor never gets into an invalid RPS value.  We
    // should never see anything with an RPS value of 0xc0000000.
    assert_eq!(
        CyclicTimestamp(0 as u32),
        CyclicTimestamp(0xbfffffff as u32).succ()
    )
}

#[test]
fn test_u64_wrap() {
    // Test that successor never gets into an invalid RPS value.  We
    // should never see anything with an RPS value of 0xc000000000000000.
    assert_eq!(
        CyclicTimestamp(0 as u64),
        CyclicTimestamp(0xbfffffffffffffff as u64).succ()
    )
}

#[test]
fn test_u128_wrap() {
    // Test that successor never gets into an invalid RPS value.  We
    // should never see anything with an RPS value of
    // 0xc0000000000000000000000000000000.
    assert_eq!(
        CyclicTimestamp(0 as u128),
        CyclicTimestamp(0xbfffffffffffffffffffffffffffffff as u128).succ()
    )
}

#[test]
fn test_u8() {
    // There are three main regions, corresponding to the three RPS
    // values: 0x00-0x3f, 0x40-0x7f, 0x80-0xbf.
    let values: [CyclicTimestamp<u8>; 9] = [
        CyclicTimestamp(0x00),
        CyclicTimestamp(0x20),
        CyclicTimestamp(0x3f),
        CyclicTimestamp(0x40),
        CyclicTimestamp(0x60),
        CyclicTimestamp(0x7f),
        CyclicTimestamp(0x80),
        CyclicTimestamp(0xa0),
        CyclicTimestamp(0xbf),
    ];

    for value in values.iter() {
        let mut other = *value;

        assert!(!value.earlier(&other));
        assert!(!value.later(&other));

        for _ in 0..0x10 {
            other = other.succ();
            assert!(value.earlier(&other));
            assert!(!other.earlier(value));
            assert!(other.later(value));
            assert!(!value.later(&other));
        }
    }
}

#[test]
fn test_u16() {
    // There are three main regions, corresponding to the three RPS
    // values: 0x0000-0x3fff, 0x4000-0x7fff, 0x8000-0xbfff.
    let values: [CyclicTimestamp<u16>; 9] = [
        CyclicTimestamp(0x0000),
        CyclicTimestamp(0x2000),
        CyclicTimestamp(0x3fff),
        CyclicTimestamp(0x4000),
        CyclicTimestamp(0x6000),
        CyclicTimestamp(0x7fff),
        CyclicTimestamp(0x8000),
        CyclicTimestamp(0xa000),
        CyclicTimestamp(0xbfff),
    ];

    for value in values.iter() {
        let mut other = *value;

        assert!(!value.earlier(&other));
        assert!(!value.later(&other));

        for _ in 0..0x1000 {
            other = other.succ();
            assert!(value.earlier(&other));
            assert!(!other.earlier(value));
            assert!(other.later(value));
            assert!(!value.later(&other));
        }
    }
}

#[test]
fn test_u32() {
    // There are three main regions, corresponding to the three RPS
    // values: 0x00000000-0x3fffffff, 0x40000000-0x7fffffff,
    // 0x80000000-0xbfffffff.
    let values: [CyclicTimestamp<u32>; 9] = [
        CyclicTimestamp(0x00000000),
        CyclicTimestamp(0x20000000),
        CyclicTimestamp(0x3fffffff),
        CyclicTimestamp(0x40000000),
        CyclicTimestamp(0x60000000),
        CyclicTimestamp(0x7fffffff),
        CyclicTimestamp(0x80000000),
        CyclicTimestamp(0xa0000000),
        CyclicTimestamp(0xbfffffff),
    ];

    for value in values.iter() {
        let mut other = *value;

        assert!(!value.earlier(&other));
        assert!(!value.later(&other));

        for _ in 0..0x1000 {
            other = other.succ();
            assert!(value.earlier(&other));
            assert!(!other.earlier(value));
            assert!(other.later(value));
            assert!(!value.later(&other));
        }
    }
}

#[test]
fn test_u64() {
    // There are three main regions, corresponding to the three RPS
    // values: 0x0000000000000000-0x3fffffffffffffff,
    // 0x4000000000000000-0x7fffffffffffffff,
    // 0x8000000000000000-0xbfffffffffffffff.
    let values: [CyclicTimestamp<u64>; 9] = [
        CyclicTimestamp(0x0000000000000000),
        CyclicTimestamp(0x2000000000000000),
        CyclicTimestamp(0x3fffffffffffffff),
        CyclicTimestamp(0x4000000000000000),
        CyclicTimestamp(0x6000000000000000),
        CyclicTimestamp(0x7fffffffffffffff),
        CyclicTimestamp(0x8000000000000000),
        CyclicTimestamp(0xa000000000000000),
        CyclicTimestamp(0xbfffffffffffffff),
    ];

    for value in values.iter() {
        let mut other = *value;

        assert!(!value.earlier(&other));
        assert!(!value.later(&other));

        for _ in 0..0x1000 {
            other = other.succ();
            assert!(value.earlier(&other));
            assert!(!other.earlier(value));
            assert!(other.later(value));
            assert!(!value.later(&other));
        }
    }
}

#[test]
fn test_u128() {
    // There are three main regions, corresponding to the three RPS
    // values:
    // 0x00000000000000000000000000000000-0x3fffffffffffffffffffffffffffffff,
    // 0x40000000000000000000000000000000-0x7fffffffffffffffffffffffffffffff,
    // 0x80000000000000000000000000000000-0xbfffffffffffffffffffffffffffffff.
    let values: [CyclicTimestamp<u128>; 9] = [
        CyclicTimestamp(0x00000000000000000000000000000000),
        CyclicTimestamp(0x20000000000000000000000000000000),
        CyclicTimestamp(0x3fffffffffffffffffffffffffffffff),
        CyclicTimestamp(0x40000000000000000000000000000000),
        CyclicTimestamp(0x60000000000000000000000000000000),
        CyclicTimestamp(0x7fffffffffffffffffffffffffffffff),
        CyclicTimestamp(0x80000000000000000000000000000000),
        CyclicTimestamp(0xa0000000000000000000000000000000),
        CyclicTimestamp(0xbfffffffffffffffffffffffffffffff),
    ];

    for value in values.iter() {
        let mut other = *value;

        assert!(!value.earlier(&other));
        assert!(!value.later(&other));

        for _ in 0..0x1000 {
            other = other.succ();
            assert!(value.earlier(&other));
            assert!(!other.earlier(value));
            assert!(other.later(value));
            assert!(!value.later(&other));
        }
    }
}

#[test]
fn test_u8_limit() {
    // Anything in an RPS group should be less than anything in the
    // next RPS group.
    //
    // So, anything from 0x00-0x3f should be less than 0x40-0x7f,
    // anything from 0x40-0x7f should be less than 0x80-0xbf,
    // anything from 0x80-0xbf should be less than 0x00-0x3f
    let values: [(CyclicTimestamp<u8>, CyclicTimestamp<u8>); 18] = [
        (CyclicTimestamp(0x00), CyclicTimestamp(0x40)),
        (CyclicTimestamp(0x00), CyclicTimestamp(0x7f)),
        (CyclicTimestamp(0x20), CyclicTimestamp(0x40)),
        (CyclicTimestamp(0x20), CyclicTimestamp(0x7f)),
        (CyclicTimestamp(0x3f), CyclicTimestamp(0x40)),
        (CyclicTimestamp(0x3f), CyclicTimestamp(0x7f)),
        (CyclicTimestamp(0x40), CyclicTimestamp(0x80)),
        (CyclicTimestamp(0x40), CyclicTimestamp(0xbf)),
        (CyclicTimestamp(0x60), CyclicTimestamp(0x80)),
        (CyclicTimestamp(0x60), CyclicTimestamp(0xbf)),
        (CyclicTimestamp(0x7f), CyclicTimestamp(0x80)),
        (CyclicTimestamp(0x7f), CyclicTimestamp(0xbf)),
        (CyclicTimestamp(0x80), CyclicTimestamp(0x00)),
        (CyclicTimestamp(0x80), CyclicTimestamp(0x3f)),
        (CyclicTimestamp(0xa0), CyclicTimestamp(0x00)),
        (CyclicTimestamp(0xa0), CyclicTimestamp(0x3f)),
        (CyclicTimestamp(0xbf), CyclicTimestamp(0x00)),
        (CyclicTimestamp(0xbf), CyclicTimestamp(0x3f)),
    ];

    for (less, greater) in values.iter() {
        assert!(less.earlier(greater));
        assert!(greater.later(less));
    }
}

#[test]
fn test_u16_limit() {
    // Anything in an RPS group should be less than anything in the
    // next RPS group.
    //
    // So, anything from 0x0000-0x3fff should be less than 0x4000-0x7fff,
    // anything from 0x4000-0x7fff should be less than 0x8000-0xbfff,
    // anything from 0x8000-0xbfff should be less than 0x0000-0x3fff
    let values: [(CyclicTimestamp<u16>, CyclicTimestamp<u16>); 18] = [
        (CyclicTimestamp(0x0000), CyclicTimestamp(0x4000)),
        (CyclicTimestamp(0x0000), CyclicTimestamp(0x7fff)),
        (CyclicTimestamp(0x2000), CyclicTimestamp(0x4000)),
        (CyclicTimestamp(0x2000), CyclicTimestamp(0x7fff)),
        (CyclicTimestamp(0x3fff), CyclicTimestamp(0x4000)),
        (CyclicTimestamp(0x3fff), CyclicTimestamp(0x7fff)),
        (CyclicTimestamp(0x4000), CyclicTimestamp(0x8000)),
        (CyclicTimestamp(0x4000), CyclicTimestamp(0xbfff)),
        (CyclicTimestamp(0x6000), CyclicTimestamp(0x8000)),
        (CyclicTimestamp(0x6000), CyclicTimestamp(0xbfff)),
        (CyclicTimestamp(0x7fff), CyclicTimestamp(0x8000)),
        (CyclicTimestamp(0x7fff), CyclicTimestamp(0xbfff)),
        (CyclicTimestamp(0x8000), CyclicTimestamp(0x0000)),
        (CyclicTimestamp(0x8000), CyclicTimestamp(0x3fff)),
        (CyclicTimestamp(0xa000), CyclicTimestamp(0x0000)),
        (CyclicTimestamp(0xa000), CyclicTimestamp(0x3fff)),
        (CyclicTimestamp(0xbfff), CyclicTimestamp(0x0000)),
        (CyclicTimestamp(0xbfff), CyclicTimestamp(0x3fff)),
    ];

    for (less, greater) in values.iter() {
        assert!(less.earlier(greater));
        assert!(greater.later(less));
    }
}

#[test]
fn test_u32_limit() {
    // Anything in an RPS group should be less than anything in the
    // next RPS group.
    //
    // So, anything from 0x00000000-0x3fffffff should be less than
    // 0x40000000-0x7fffffff, anything from 0x40000000-0x7fffffff should
    // be less than 0x80000000-0xbfffffff, anything from
    // 0x80000000-0xbfffffff should be less than 0x00000000-0x3fffffff
    let values: [(CyclicTimestamp<u32>, CyclicTimestamp<u32>); 18] = [
        (CyclicTimestamp(0x00000000), CyclicTimestamp(0x40000000)),
        (CyclicTimestamp(0x00000000), CyclicTimestamp(0x7fffffff)),
        (CyclicTimestamp(0x20000000), CyclicTimestamp(0x40000000)),
        (CyclicTimestamp(0x20000000), CyclicTimestamp(0x7fffffff)),
        (CyclicTimestamp(0x3fffffff), CyclicTimestamp(0x40000000)),
        (CyclicTimestamp(0x3fffffff), CyclicTimestamp(0x7fffffff)),
        (CyclicTimestamp(0x40000000), CyclicTimestamp(0x80000000)),
        (CyclicTimestamp(0x40000000), CyclicTimestamp(0xbfffffff)),
        (CyclicTimestamp(0x60000000), CyclicTimestamp(0x80000000)),
        (CyclicTimestamp(0x60000000), CyclicTimestamp(0xbfffffff)),
        (CyclicTimestamp(0x7fffffff), CyclicTimestamp(0x80000000)),
        (CyclicTimestamp(0x7fffffff), CyclicTimestamp(0xbfffffff)),
        (CyclicTimestamp(0x80000000), CyclicTimestamp(0x00000000)),
        (CyclicTimestamp(0x80000000), CyclicTimestamp(0x3fffffff)),
        (CyclicTimestamp(0xa0000000), CyclicTimestamp(0x00000000)),
        (CyclicTimestamp(0xa0000000), CyclicTimestamp(0x3fffffff)),
        (CyclicTimestamp(0xbfffffff), CyclicTimestamp(0x00000000)),
        (CyclicTimestamp(0xbfffffff), CyclicTimestamp(0x3fffffff)),
    ];

    for (less, greater) in values.iter() {
        assert!(less.earlier(greater));
        assert!(greater.later(less));
    }
}

#[test]
fn test_u64_limit() {
    // Anything in an RPS group should be less than anything in the
    // next RPS group.
    //
    // So, anything from 0x0000000000000000-0x3fffffffffffffff should be less
    // than 0x4000000000000000-0x7fffffffffffffff, anything from
    // 0x4000000000000000-0x7fffffffffffffff should be less than
    // 0x8000000000000000-0xbfffffffffffffff, anything from
    // 0x8000000000000000-0xbfffffffffffffff should be less than
    // 0x0000000000000000-0x3fffffffffffffff
    let values: [(CyclicTimestamp<u64>, CyclicTimestamp<u64>); 18] = [
        (
            CyclicTimestamp(0x0000000000000000),
            CyclicTimestamp(0x4000000000000000),
        ),
        (
            CyclicTimestamp(0x0000000000000000),
            CyclicTimestamp(0x7fffffffffffffff),
        ),
        (
            CyclicTimestamp(0x2000000000000000),
            CyclicTimestamp(0x4000000000000000),
        ),
        (
            CyclicTimestamp(0x2000000000000000),
            CyclicTimestamp(0x7fffffffffffffff),
        ),
        (
            CyclicTimestamp(0x3fffffffffffffff),
            CyclicTimestamp(0x4000000000000000),
        ),
        (
            CyclicTimestamp(0x3fffffffffffffff),
            CyclicTimestamp(0x7fffffffffffffff),
        ),
        (
            CyclicTimestamp(0x4000000000000000),
            CyclicTimestamp(0x8000000000000000),
        ),
        (
            CyclicTimestamp(0x4000000000000000),
            CyclicTimestamp(0xbfffffffffffffff),
        ),
        (
            CyclicTimestamp(0x6000000000000000),
            CyclicTimestamp(0x8000000000000000),
        ),
        (
            CyclicTimestamp(0x6000000000000000),
            CyclicTimestamp(0xbfffffffffffffff),
        ),
        (
            CyclicTimestamp(0x7fffffffffffffff),
            CyclicTimestamp(0x8000000000000000),
        ),
        (
            CyclicTimestamp(0x7fffffffffffffff),
            CyclicTimestamp(0xbfffffffffffffff),
        ),
        (
            CyclicTimestamp(0x8000000000000000),
            CyclicTimestamp(0x0000000000000000),
        ),
        (
            CyclicTimestamp(0x8000000000000000),
            CyclicTimestamp(0x3fffffffffffffff),
        ),
        (
            CyclicTimestamp(0xa000000000000000),
            CyclicTimestamp(0x0000000000000000),
        ),
        (
            CyclicTimestamp(0xa000000000000000),
            CyclicTimestamp(0x3fffffffffffffff),
        ),
        (
            CyclicTimestamp(0xbfffffffffffffff),
            CyclicTimestamp(0x0000000000000000),
        ),
        (
            CyclicTimestamp(0xbfffffffffffffff),
            CyclicTimestamp(0x3fffffffffffffff),
        ),
    ];

    for (less, greater) in values.iter() {
        assert!(less.earlier(greater));
        assert!(greater.later(less));
    }
}

#[test]
fn test_u128_limit() {
    // Anything in an RPS group should be less than anything in the
    // next RPS group.
    //
    // So, anything from 0x00000000000000000000000000000000 -
    // 0x3fffffffffffffffffffffffffffffff should be less than
    // 0x40000000000000000000000000000000 -
    // 0x7fffffffffffffffffffffffffffffff, anything from
    // 0x40000000000000000000000000000000 -
    // 0x7fffffffffffffffffffffffffffffff should be less than
    // 0x80000000000000000000000000000000 -
    // 0xbfffffffffffffffffffffffffffffff, anything from
    // 0x80000000000000000000000000000000 -
    // 0xbfffffffffffffffffffffffffffffff should be less than
    // 0x00000000000000000000000000000000 -
    // 0x3fffffffffffffffffffffffffffffff
    let values: [(CyclicTimestamp<u128>, CyclicTimestamp<u128>); 18] = [
        (
            CyclicTimestamp(0x00000000000000000000000000000000),
            CyclicTimestamp(0x40000000000000000000000000000000),
        ),
        (
            CyclicTimestamp(0x00000000000000000000000000000000),
            CyclicTimestamp(0x7fffffffffffffffffffffffffffffff),
        ),
        (
            CyclicTimestamp(0x20000000000000000000000000000000),
            CyclicTimestamp(0x40000000000000000000000000000000),
        ),
        (
            CyclicTimestamp(0x20000000000000000000000000000000),
            CyclicTimestamp(0x7fffffffffffffffffffffffffffffff),
        ),
        (
            CyclicTimestamp(0x40000000000000000000000000000000),
            CyclicTimestamp(0x7fffffffffffffffffffffffffffffff),
        ),
        (
            CyclicTimestamp(0x3fffffffffffffffffffffffffffffff),
            CyclicTimestamp(0x7fffffffffffffffffffffffffffffff),
        ),
        (
            CyclicTimestamp(0x40000000000000000000000000000000),
            CyclicTimestamp(0x80000000000000000000000000000000),
        ),
        (
            CyclicTimestamp(0x40000000000000000000000000000000),
            CyclicTimestamp(0xbfffffffffffffffffffffffffffffff),
        ),
        (
            CyclicTimestamp(0x40000000000000000000000000000000),
            CyclicTimestamp(0x80000000000000000000000000000000),
        ),
        (
            CyclicTimestamp(0x60000000000000000000000000000000),
            CyclicTimestamp(0xbfffffffffffffffffffffffffffffff),
        ),
        (
            CyclicTimestamp(0x7fffffffffffffffffffffffffffffff),
            CyclicTimestamp(0x80000000000000000000000000000000),
        ),
        (
            CyclicTimestamp(0x7fffffffffffffffffffffffffffffff),
            CyclicTimestamp(0xbfffffffffffffffffffffffffffffff),
        ),
        (
            CyclicTimestamp(0x80000000000000000000000000000000),
            CyclicTimestamp(0x00000000000000000000000000000000),
        ),
        (
            CyclicTimestamp(0x80000000000000000000000000000000),
            CyclicTimestamp(0x3fffffffffffffffffffffffffffffff),
        ),
        (
            CyclicTimestamp(0xa0000000000000000000000000000000),
            CyclicTimestamp(0x00000000000000000000000000000000),
        ),
        (
            CyclicTimestamp(0xa0000000000000000000000000000000),
            CyclicTimestamp(0x3fffffffffffffffffffffffffffffff),
        ),
        (
            CyclicTimestamp(0xbfffffffffffffffffffffffffffffff),
            CyclicTimestamp(0x00000000000000000000000000000000),
        ),
        (
            CyclicTimestamp(0xbfffffffffffffffffffffffffffffff),
            CyclicTimestamp(0x3fffffffffffffffffffffffffffffff),
        ),
    ];

    for (less, greater) in values.iter() {
        assert!(less.earlier(greater));
        assert!(greater.later(less));
    }
}

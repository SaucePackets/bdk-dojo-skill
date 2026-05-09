#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeeRate {
    pub sat_per_vb: u64,
}

impl FeeRate {
    pub fn from_sat_per_vb(sat_per_vb: u64) -> Self {
        Self { sat_per_vb }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxSizeEstimate {
    pub vbytes: u64,
}

impl TxSizeEstimate {
    pub fn new(vbytes: u64) -> Self {
        Self { vbytes }
    }
}

pub fn fee(size: TxSizeEstimate, rate: FeeRate) -> u64 {
    size.vbytes * rate.sat_per_vb
}

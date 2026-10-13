use crate::*;
use bitflags::bitflags;

#[repr(usize)]
pub enum OperationType {
    None,
    Map,
    Unmap,
    GetUnsetDepth,
}

bitflags! {
    /// Memory attributes passed to an address-space map operation.
    ///
    /// Bits 0–2 contain read, write, and execute permissions. Bit 3 is reserved
    /// and must be zero. Bits 4–7 encode the memory protection type:
    /// 0 = Normal, 1 = Buffered, 2 = Uncached.
    ///
    /// Combine permissions with one memory protection type. `BUFFERED` and
    /// `UNCACHED` are mutually exclusive; omitting both selects `NORMAL`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Attribute: usize
    {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXECUTE = 1 << 2;
        /// All access permissions, with Normal memory protection.
        const ALL = (1 << 0) | (1 << 1) | (1 << 2);
        /// bit 3 is currently reserved and must be zero.
        /// Memory protection types. These are mutually exclusive; omit both to select Normal.
        const NORMAL = 0 << 4;
        const BUFFERED = 1 << 4;
        const UNCACHED = 2 << 4;
    }
}

#[cfg(test)]
mod tests {
    use super::Attribute;

    #[test]
    fn memory_attributes_match_kernel_abi() {
        assert_eq!(Attribute::READ.bits(), 0x01);
        assert_eq!(Attribute::WRITE.bits(), 0x02);
        assert_eq!(Attribute::EXECUTE.bits(), 0x04);
        assert_eq!((Attribute::ALL | Attribute::NORMAL).bits(), 0x07);
        assert_eq!((Attribute::ALL | Attribute::BUFFERED).bits(), 0x17);
        assert_eq!((Attribute::ALL | Attribute::UNCACHED).bits(), 0x27);

        for bits in [0x07, 0x17, 0x27] {
            assert_eq!(Attribute::from_bits(bits).unwrap().bits(), bits);
            assert!(Attribute::from_bits(bits | 0x08).is_none());
        }
    }
}

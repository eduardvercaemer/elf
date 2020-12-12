//! Regarding program headers (segments).

/// The posible types for a segment.
enum Type {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Tls,
    Unhandled,
}

/// Contains the data from the segment flags.
struct Flags {
}

/// Represents a whole segment entry.
pub struct Segment {
    etype:      Type,       // 32-bits
    flags:      Flags,      // 32-bits
    offset:     u64,        // 64-bits
    pub vaddr:  u64,        // 64-bits
    pub paddr:  u64,        // 64-bits
    filesz:     u64,        // 64-bits
    memsz:      u64,        // 64-bits
    pub align:  u64,        // 64-bits
}

/// Simple type methods.
impl Type {
    /// Default type.
    pub fn empty() -> Self {
        Self::Unhandled
    }

    /// Type from real value.
    pub fn new(_etype: u32) -> Self {
        Self::Unhandled
    }
}

/// Simple flag methods.
impl Flags {
    /// Default flags
    pub fn empty() -> Self {
        Self {
        }
    }

    /// Flags from real value.
    pub fn new(_flags: u32) -> Self {
        Self {
        }
    }
}

/// Simple segment methods.
impl Segment {
    /// Default segment header
    pub fn empty() -> Self {
        Self {
            etype:      Type::empty(),
            flags:      Flags::empty(),
            offset:     0,
            vaddr:      0,
            paddr:      0,
            filesz:     0,
            memsz:      0,
            align:      0,
        }
    }

    /// Get string representation of segment type.
    pub fn type_str(&self) -> &'static str {
        self.etype.as_str()
    }
}

/// Format methods.
mod format {
    use super::*;

    impl Type {
        /// Get string slice representation of type.
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Null      => "null",
                Self::Load      => "loadable segment",
                Self::Dynamic   => "dynamic linking info",
                Self::Interp    => "interpreter",
                Self::Note      => "aux info",
                Self::Shlib     => "reserved",
                Self::Phdr      => "header entry",
                Self::Tls       => "tls",
                Self::Unhandled => "unhandled",
            }
        }
    }
}

/// File IO methods.
mod io {
    use super::*;
    use std::fs::File;
    use crate::util;

    impl Segment {
        /// Extract a segment from a file at current position.
        pub fn extract(file: &mut File) -> Self {
            let mut new = Self::empty();

            new.etype  = Type::new(util::read_u32(file));
            new.flags  = Flags::new(util::read_u32(file));
            new.offset = util::read_u64(file);
            new.vaddr  = util::read_u64(file);
            new.paddr  = util::read_u64(file);
            new.filesz = util::read_u64(file);
            new.memsz  = util::read_u64(file);
            new.align  = util::read_u64(file);

            new
        }
    }
}

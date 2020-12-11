//! Relevant to a symbol entry in an ELF file symbol table.
//!
//! **TODO:
//! - better documentation.**

/// Posible symbol types.
/// Obtained from the lower 4 bits of the info byte.
#[derive(PartialEq)]
enum Type {
    NoType,
    Object,
    Func,
    Section,
    File,
    Common,
    TLS,
    Num,
    Unhandled,
}

/// Posible symbol bindings.
/// Obtained from the higher 4 bits of the info byte.
#[derive(PartialEq)]
enum Bind {
    Local,
    Global,
    Weak,
    Unhandled,
}

/// Represents an individual entry in a symbol table.
pub struct Sym {
    /// Index into the symbol string table.
    pub nameoff:    usize,      // 32-bits
    etype:          Type,       // \_ 8-bits
    bind:           Bind,       // /
    other:          u8,         // 8-bits
    pub shndx:      usize,      // 16-bits
    pub value:      u64,        // 64-bits
    size:           u64,        // 64-bits

    /// Extracted name string.
    pub name:       Option<String>,
}

/// Simple type methods.
impl Type {
    /// Default type.
    pub fn empty() -> Self {
        Self::Unhandled
    }

    /// Get type from value of `info`.
    ///
    /// The type is contained in the lower 4-bits of
    /// `info`.
    pub fn new(info: u8) -> Self {
        match info & 0x0f {
            0 => Self::NoType,
            1 => Self::Object,
            2 => Self::Func,
            3 => Self::Section,
            4 => Self::File,
            _ => Self::Unhandled,
        }
    }

    /// String slice representation of the type.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoType    => "no type",
            Self::Object    => "object",
            Self::Func      => "function",
            Self::Section   => "section",
            Self::File      => "file",
            Self::Common    => "common",
            Self::TLS       => "tls",
            Self::Num       => "num",
            Self::Unhandled => "unhandled",
        }
    }

}

/// Simple bind methods.
impl Bind {
    /// Default bind.
    pub fn empty() -> Self {
        Self::Unhandled
    }

    /// Get bind from value of info.
    ///
    /// The bind is contained in the higher 4-bits of
    /// `info`.
    pub fn new(info: u8) -> Self {
        match info >> 4 {
            0 => Self::Local,
            1 => Self::Global,
            2 => Self::Weak,
            _ => Self::Unhandled,
        }
    }

    /// Get a string slice representation of the bind.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local     => "local",
            Self::Global    => "global",
            Self::Weak      => "weak",
            Self::Unhandled => "unhandled",
        }
    }
}

/// Simple sym methods.
impl Sym {
    /// Default sym.
    pub fn empty() -> Self {
        Self {
            nameoff:    0,
            etype:      Type::empty(),
            bind:       Bind::empty(),
            other:      0,
            shndx:      0,
            value:      0,
            size:       0,

            name:       None,
        }
    }

    /// Get the symbols type string.
    pub fn type_str(&self) -> &'static str {
        self.etype.as_str()
    }

    /// Get the symbols bind string.
    pub fn bind_str(&self) -> &'static str {
        self.bind.as_str()
    }

    /// Wether the symbol represents a section.
    pub fn is_section(&self) -> bool {
        self.etype == Type::Section
    }
}

/// Format methods.
mod format {
    use std::fmt;
    use super::*;

    impl fmt::Display for Type {
        /// Convert our symbol type into a string.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = self.as_str();
            write!(f, "{}", s)
        }
    }

    impl fmt::Display for Bind {
        /// Convert our symbol binding into a string.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = self.as_str();
            write!(f, "{}", s)
        }
    }
}

/// File IO methods.
pub mod io {
    use super::*;
    use std::fs::File;
    use super::super::util;

    impl Sym {
        /// Extract a symbol from a file **at current offset**.
        pub fn extract(file: &mut File) -> Self {
            let mut new = Self::empty();

            new.nameoff = util::read_u32(file) as usize;
            let info    = util::read_u8(file);
            new.etype   = Type::new(info);
            new.bind    = Bind::new(info);
            new.other   = util::read_u8(file);
            new.shndx   = util::read_u16(file) as usize;
            new.value   = util::read_u64(file);
            new.size    = util::read_u64(file);

            new
        }
    }
}


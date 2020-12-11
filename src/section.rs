//! Relevant to the sections contained in an ELF file.
//!
//! **TODO:
//! - better documentation.**

/// Posible section types.
#[derive(PartialEq)]
enum Type {
    Null,
    Progbits,
    Symtab,
    Strtab,
    Rela,
    Hash,
    Dynamic,
    Note,
    Nobits,
    Rel,
    Shlib,
    Dynsym,
    Unhandled,
}

/// Represents a whole section in an ELF file.
pub struct Section {
    /// Index into shstrtab for this section's name.
    pub nameoff:    usize,      // 32-bits
    /// Indicates the type of this section.
    etype:          Type,       // 32-bits
    flags:          u64,        // 64-bits
    addr:           u64,        // 64-bits      
    pub offset:     u64,        // 64-bits
    pub size:       u64,        // 64-bits
    link:           usize,      // 32-bits
    info:           u32,        // 32-bits
    addralign:      usize,      // 64-bits
    pub entsize:    u64,        // 64-bits

    /// Extracted name string.
    pub name:       Option<String>,
}

impl Type {
    /// Default type.
    pub fn empty() -> Self {
        Self::Unhandled
    }

    /// String slice representation for type.
    fn as_str(&self) -> &'static str {
        match self {
            Self::Null     => "null",
            Self::Progbits => "progbits",
            Self::Symtab   => "symtab",
            Self::Strtab   => "strtab",
            Self::Rela     => "rela",
            Self::Hash     => "hash",
            Self::Dynamic  => "dynamic",
            Self::Note     => "note",
            Self::Nobits   => "nobits",
            Self::Rel      => "rel",
            Self::Shlib    => "shlib",
            Self::Dynsym   => "dynsym",
            _              => "unhandled",
        }
    }

    /// Create type from value.
    /// **TODO: missing type parsing.**
    pub fn new(etype: u32) -> Self {
        match etype {
            0  => Self::Null,
            1  => Self::Progbits,
            2  => Self::Symtab,
            3  => Self::Strtab,
            4  => Self::Rela,
            5  => Self::Hash,
            6  => Self::Dynamic,
            7  => Self::Note,
            8  => Self::Nobits,
            9  => Self::Rel,
            10 => Self::Shlib,
            11 => Self::Dynsym,
            _  => Self::Unhandled,
        }
    }
}

/// Simple section methods.
impl Section {
    /// Default section.
    pub fn empty() -> Self {
        Self {
            nameoff:    0,
            etype:      Type::empty(),
            flags:      0,
            addr:       0,
            offset:     0,
            size:       0,
            link:       0,
            info:       0,
            addralign:  0,
            entsize:    0,

            name:       None,
        }
    }

    /// Get string slice for section type.
    pub fn type_str(&self) -> &'static str {
        self.etype.as_str()
    }

    /// Check if the section is a symbol table.
    pub fn is_symtab(&self) -> bool {
        self.etype == Type::Symtab
    }

    /// Check if the section is a string table.
    pub fn is_strtab(&self) -> bool {
        self.etype == Type::Strtab
    }
}

/// Format methods.
mod format {
    use std::fmt;
    use super::*;

    impl fmt::Display for Type {
        /// Convert a section type to string.
        /// **TODO: add rest of types.**
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = self.as_str();
            write!(f, "{}", s)
        }
    }
}

/// File IO methods.
mod io {
    use std::fs::File;
    use super::super::util;
    use super::*;

    impl Section {
        /// Extract section from file **at current offset**
        pub fn extract(file: &mut File) -> Self {
            let mut new = Self::empty();

            new.nameoff   = util::read_u32(file) as usize;
            new.etype     = Type::new(util::read_u32(file));
            new.flags     = util::read_u64(file) as u64;
            new.addr      = util::read_u64(file) as u64;
            new.offset    = util::read_u64(file) as u64;
            new.size      = util::read_u64(file) as u64;
            new.link      = util::read_u32(file) as usize;
            new.info      = util::read_u32(file) as u32;
            new.addralign = util::read_u64(file) as usize;
            new.entsize   = util::read_u64(file) as u64;

            new
        }
    }
}


//! Relevant to the main ELF header of an object file.
//!
//! **TODO:
//! better documentation of header layout.**

/// ELF header identification.
struct Ident {
    /// Wether the header is a valid ELF header.
    valid: bool,
}

/// ELF file type.
enum Type {
    /// An unknown type.
    Null,
    /// A relocatable file.
    Rel,
    /// An executable.
    Exec,
    /// A shared object.
    Dyn,
    /// A core file.
    Core,
}

/// ELF machine.
/// **TODO:**
struct Machine {
}

/// ELF version.
/// **TODO:**
struct Version {
}

/// ELF flags.
/// **TODO:**
struct Flags {
}

/// ELF header struct.
pub struct Header {
    ident:          Ident,      // 16-bits
    etype:          Type,       // 16-bits
    machine:        Machine,    // 16-bits
    version:        Version,    // 32-bits
    pub entry:      u64,        // 64-bits
    pub phoff:      u64,        // 64-bits
    pub shoff:      u64,        // 64-bits
    flags:          Flags,      // 32-bits
    ehsize:         u16,        // 16-bits
    pub phentsize:  u16,        // 16-bits
    pub phnum:      u16,        // 16-bits
    pub shentsize:  u16,        // 16-bits
    pub shnum:      u16,        // 16-bits
    pub shstrndx:   usize,      // 16-bits
}

/// Simple Ident methods.
impl Ident {
    /// Default Ident object.
    pub fn empty() -> Self {
        Self {
            valid: false,
        }
    }

    /// Generate Ident object from ident bytes.
    pub fn new(ident: [u8; 16]) -> Self {
        let valid = ident[0] == 0x7f &&
                        ident[1] == b'E' &&
                        ident[2] == b'L' &&
                        ident[3] == b'F';
        Self {
            valid,
        }
    }
}

/// Simple Type methods.
impl Type {
    /// Default Type object.
    pub fn empty() -> Self {
        Self::Null
    }

    /// Generate Type object from type value.
    pub fn new(etype: u16) -> Self {
        match etype {
            0 => Self::Null,
            1 => Self::Rel,
            2 => Self::Exec,
            3 => Self::Dyn,
            4 => Self::Core,
            _ => Self::Null,
        }
    }

    /// Get string slice representation of the type.
    pub fn as_str(&self) -> &'static str {
            match self {
                Self::Null => "unknown",
                Self::Rel  => "relocatable",
                Self::Exec => "executable",
                Self::Dyn  => "shared object",
                Self::Core => "core",
            }
    }
}

/// Simple Machine methods.
impl Machine {
    /// Default Machine object.
    pub fn empty() -> Self {
        Self {
        }
    }

    /// Generate Machine object from machine value.
    pub fn new(_machine: u16) -> Self {
        Self {
        }
    }
}

/// Simple Version methods.
impl Version {
    /// Default Version object.
    pub fn empty() -> Self{
        Self {
        }
    }

    /// Generate Version object from version value.
    pub fn new(_version: u32) -> Self {
        Self {
        }
    }
}

/// Simple Flags methods.
impl Flags {
    /// Default Flags object.
    pub fn empty() -> Self{
        Self {
        }
    }

    /// Generate Flags object from flags value.
    pub fn new(_flags: u32) -> Self {
        Self {
        }
    }
}

/// Simple header methods.
impl Header {
    /// Creates an empty header.
    pub fn empty() -> Self {
        Self {
            ident:      Ident::empty(),
            etype:      Type::empty(),
            machine:    Machine::empty(),
            version:    Version::empty(),
            entry:      0,
            phoff:      0,
            shoff:      0,
            flags:      Flags::empty(),
            ehsize:     0,
            phentsize:  0,
            phnum:      0,
            shentsize:  0,
            shnum:      0,
            shstrndx:   0,
        }
    }

    /// Check if an ELF header is valid.
    pub fn valid(&self) -> bool {
        self.ident.valid
    }

    /// Get string slice for header type.
    pub fn type_str(&self) -> &'static str {
        self.etype.as_str()
    }
}

/// Format methods.
pub mod format {
    use std::fmt;
    use super::*;

    impl fmt::Display for Type {
        /// Convert header type to string.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = self.as_str();
            write!(f, "{}", s)
        }
    }

}

/// File IO methods.
pub mod io {
    use std::fs::File;
    use std::io::{Seek,Read,SeekFrom};
    use super::*;
    use super::super::util;

    impl Header {
        /// Extract ELF header from file.
        pub fn extract(file: &mut File) -> Self {
            let mut new = Self::empty();

            // go to start of file
            file.seek(SeekFrom::Start(0)).unwrap();
            
            // ident
            let mut ident = [0u8; 16];
            file.read(&mut ident).unwrap();
            new.ident     = Ident::new(ident);
            new.etype     = Type::new(util::read_u16(file));
            new.machine   = Machine::new(util::read_u16(file));
            new.version   = Version::new(util::read_u32(file));
            new.entry     = util::read_u64(file) as u64;
            new.phoff     = util::read_u64(file) as u64;
            new.shoff     = util::read_u64(file) as u64;
            new.flags     = Flags::new(util::read_u32(file));
            new.ehsize    = util::read_u16(file) as u16;
            new.phentsize = util::read_u16(file) as u16;
            new.phnum     = util::read_u16(file) as u16;
            new.shentsize = util::read_u16(file) as u16;
            new.shnum     = util::read_u16(file) as u16;
            new.shstrndx  = util::read_u16(file) as usize;

            new
        }
    }
}


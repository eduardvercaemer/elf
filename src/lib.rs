/// Utility IO methods
pub mod util {
    use std::fs::File;
    use std::io::Read;
    /// Read one byte.
    pub fn read_u8(file: &mut File) -> u8 {
        let mut buf = [0u8; 1];
        file.read(&mut buf).unwrap();
        u8::from_ne_bytes(buf)
    }
    /// Read two bytes.
    pub fn read_u16(file: &mut File) -> u16 {
        let mut buf = [0u8; 2];
        file.read(&mut buf).unwrap();
        u16::from_ne_bytes(buf)
    }
    /// Read four bytes.
    pub fn read_u32(file: &mut File) -> u32 {
        let mut buf = [0u8; 4];
        file.read(&mut buf).unwrap();
        u32::from_ne_bytes(buf)
    }
    /// Read eight bytes.
    pub fn read_u64(file: &mut File) -> u64 {
        let mut buf = [0u8; 8];
        file.read(&mut buf).unwrap();
        u64::from_ne_bytes(buf)
    }
}

/// Relevant to the main ELF header.
pub mod header {
    /// ELF header identification.
    struct Ident {
        /// Wether the header is a valid ELF header.
        pub valid: bool,
    }

    /// ELF type.
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
    struct Machine {
    }

    /// ELF version.
    struct Version {
    }

    /// ELF flags.
    struct Flags {
    }

    /// ELF header struct.
    pub struct Header {
        ident: Ident,       // 16 bytes
        etype: Type,        // 16 bytes
        machine: Machine,   // 16 bytes
        version: Version,   // 32 bytes
        entry: u64,
        phoff: u64,
        shoff: u64,
        flags: Flags,       // 32 bytes
        ehsize: u16,
        phentsize: u16,
        phnum: u16,
        shentsize: u16,
        shnum: u16,
        shstrndx: u16,
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
    }

    /// Simple Machine methods.
    impl Machine {
        /// Default Machine object.
        pub fn empty() -> Self {
            Self {
            }
        }

        /// Generate Machine object from machine value.
        pub fn new(machine: u16) -> Self {
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
        pub fn new(version: u32) -> Self {
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
        pub fn new(flags: u32) -> Self {
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
    }

    /// File IO methods.
    pub mod io {
        use std::fs::File;
        use std::io::{self,Seek,Read};
        use super::*;
        use super::super::util;

        impl Header {
            /// Generate ELF header from file name.
            pub fn from_file(filename: &str) -> Self {
                let mut file = File::open(filename).unwrap();
                Self::extract(&mut file)
            }

            /// Extract ELF header from file (will seek).
            pub fn extract(file: &mut File) -> Self {
                let mut new = Self::empty();
                
                // ident
                let mut ident = [0u8; 16];
                file.read(&mut ident).unwrap();
                new.ident     = Ident::new(ident);
                new.etype     = Type::new(util::read_u16(file));
                new.machine   = Machine::new(util::read_u16(file));
                new.version   = Version::new(util::read_u32(file));
                new.entry     = util::read_u64(file);
                new.phoff     = util::read_u64(file);
                new.shoff     = util::read_u64(file);
                new.flags     = Flags::new(util::read_u32(file));
                new.ehsize    = util::read_u16(file);
                new.phentsize = util::read_u16(file);
                new.phnum     = util::read_u16(file);
                new.shentsize = util::read_u16(file);
                new.shnum     = util::read_u16(file);
                new.shstrndx  = util::read_u16(file);

                new
            }
        }
    }
}

/// Relevant to individual section headers.
pub mod section {
    /// Posible section types.
    pub enum Type {
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
        name: u32,
        /// Indicates the type of this section.
        etype: Type,
        flags: u64,
        addr: u64,
        offset: u64,
        size: u64,
        link: u32,
        info: u32,
        addralign: u64,
        entsize: u64,
    }

    impl Type {
        /// Default type.
        pub fn empty() -> Self {
            Self::Null
        }

        /// Create type from value.
        pub fn new(etype: u32) -> Self {
            match etype {
                0 => Self::Null,
                1 => Self::Progbits,
                2 => Self::Symtab,
                3 => Self::Strtab,
                _ => Self::Unhandled,
            }
        }
    }

    /// Simple section methods.
    impl Section {
        /// Default section.
        pub fn empty() -> Self {
            Self {
                name:      0,
                etype:     Type::empty(),
                flags:     0,
                addr:      0,
                offset:    0,
                size:      0,
                link:      0,
                info:      0,
                addralign: 0,
                entsize:   0,
            }
        }
    }

    /// File IO methods.
    mod io {
        use std::fs::File;
        use std::io::Read;
        use super::super::util;
        use super::*;

        impl Section {
            /// Extract section from file **at current offset**
            pub fn extract(file: &mut File) -> Self {
                let mut new = Self::empty();

                new.name      = util::read_u32(file);
                new.etype     = Type::new(util::read_u32(file));
                new.flags     = util::read_u64(file);
                new.addr      = util::read_u64(file);
                new.offset    = util::read_u64(file);
                new.size      = util::read_u64(file);
                new.link      = util::read_u32(file);
                new.info      = util::read_u32(file);
                new.addralign = util::read_u64(file);
                new.entsize   = util::read_u64(file);

                new
            }
        }
    }
}

pub mod object {
    use super::header::Header;
    use super::section::Section;

    /// Represents a whole object file.
    pub struct Object {
        /// Main ELF header.
        header: Header,
        /// Sections contained in the object file.
        sections: Vec<Section>,
    }

    /// Simple object methods.
    impl Object {
        /// Default object.
        pub fn empty() -> Self {
            Self {
                header: Header::empty(),
                sections: vec![],
            }
        }

    }

    /// File IO methods.
    mod io {
        use std::fs::File;
        use super::*;

        impl Object {
            /// Generates an object from given file.
            pub fn extract(file: &mut File) -> Self {
                /* init default object */
                let mut new = Self::empty();

                /* extract ELF header */
                new.header = Header::extract(file);

                /* make sure its a valid file */
                assert!(new.header.valid());

                new
            }

            /// Generates an object from given file name.
            pub fn from_file(filename: &str) -> Self {
                let mut file = File::open(filename).unwrap();
                Self::extract(&mut file)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::header::Header;

    #[test]
    fn magic() {
        let h = Header::from_file("/home/ed/repos/elf/samples/main.o");
        assert!(h.valid());
        let h = Header::from_file("/home/ed/repos/elf/samples/main.c");
        assert!(!h.valid());
    }
}

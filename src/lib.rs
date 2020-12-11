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
        ident:          Ident,      // 16-bits
        etype:          Type,       // 16-bits
        machine:        Machine,    // 16-bits
        version:        Version,    // 32-bits
        entry:          u64,        // 64-bits
        phoff:          u64,        // 64-bits
        pub shoff:      u64,        // 64-bits
        flags:          Flags,      // 32-bits
        ehsize:         u16,        // 16-bits
        phentsize:      u16,        // 16-bits
        phnum:          u16,        // 16-bits
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
    }

    /// Format methods.
    pub mod format {
        use std::fmt;
        use super::*;

        impl fmt::Debug for Type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s: &'static str = match self {
                    Self::Null => "unknown type",
                    Self::Rel => "relocatable file",
                    Self::Exec => "executable file",
                    Self::Dyn => "shared object file",
                    Self::Core => "core file",
                };
                write!(f, "{}", s)
            }
        }
    
        impl fmt::Debug for Header {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Header {{ ")?;
                write!(f, "type:{:?} ", self.etype)?;
                write!(f, " }}")?;
                Ok(())
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
            /// Generate ELF header from file name.
            pub fn from_file(filename: &str) -> Self {
                let mut file = File::open(filename).unwrap();
                Self::extract(&mut file)
            }

            /// Extract ELF header from file (will seek).
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
}

/// Relevant to individual section headers.
pub mod section {
    /// Posible section types.
    #[derive(PartialEq)]
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
        pub name:       usize,      // 32-bits
        /// Indicates the type of this section.
        pub etype:      Type,       // 32-bits
        flags:          u64,        // 64-bits
        addr:           u64,        // 64-bits      
        pub offset:     u64,        // 64-bits
        pub size:       u64,        // 64-bits
        link:           usize,      // 32-bits
        info:           u32,        // 32-bits
        addralign:      usize,      // 64-bits
        pub entsize:    u64,        // 64-bits
    }

    impl Type {
        /// Default type.
        pub fn empty() -> Self {
            Self::Unhandled
        }

        /// Create type from value.
        pub fn new(etype: u32) -> Self {
            match etype {
                0 => Self::Null,
                1 => Self::Progbits,
                2 => Self::Symtab,
                3 => Self::Strtab,
                /* TODO:
                 * - add rest of types
                 */
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

    /// Format methods.
    mod format {
        use std::fmt;
        use super::*;

        impl fmt::Debug for Type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s: &'static str = match self {
                    Self::Null => "null",
                    Self::Progbits => "progbits",
                    Self::Symtab => "symbol table",
                    Self::Strtab => "string table",
                    _ => "unhandled section",
                };
                write!(f, "{}", s)
            }
        }

        impl fmt::Debug for Section {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Section {{ ")?;
                write!(f, "type:{:?} ", self.etype)?;
                write!(f, "}}")?;
                Ok(())
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

                new.name      = util::read_u32(file) as usize;
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
}

/// Relevant to symbol entries.
mod sym {
    /// Posible symbol types.
    /// Ubtained from the lower 4 bits of the info byte.
    enum Type {
        NoType,
        Object,
        Func,
        Section,
        File,
        Unhandled,
    }

    /// Posible symbol bindings.
    /// Ubtained from the higher 4 bits of the info byte.
    enum Bind {
        Local,
        Global,
        Weak,
        Unhandled,
    }

    /// Represents an individual entry in a symbol table.
    pub struct Sym {
        /// Index into the symbol string table.
        pub name:   usize,      // 32-bits
        etype:      Type,       // \_ 8-bits
        bind:       Bind,       // /
        other:      u8,         // 8-bits
        shndx:      usize,      // 16-bits
        value:      u64,        // 64-bits
        size:       u64,        // 64-bits
    }

    /// Simple type methods.
    impl Type {
        /// Default type.
        pub fn empty() -> Self {
            Self::Unhandled
        }

        /// Get type from value of info.
        pub fn new(info: u8) -> Self {
            match info {
                0 => Self::NoType,
                1 => Self::Object,
                2 => Self::Func,
                3 => Self::Section,
                4 => Self::File,
                _ => Self::Unhandled,
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
        pub fn new(info: u8) -> Self {
            match info {
                0 => Self::Local,
                1 => Self::Global,
                2 => Self::Weak,
                _ => Self::Unhandled,
            }
        }
    }

    /// Simple sym methods.
    impl Sym {
        /// Default sym.
        pub fn empty() -> Self {
            Self {
                name:   0,
                etype:  Type::empty(),
                bind:   Bind::empty(),
                other:  0,
                shndx:  0,
                value:  0,
                size:   0,
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

                new.name  = util::read_u32(file) as usize;
                let info  = util::read_u8(file);
                new.etype = Type::new(info);
                new.bind  = Bind::new(info);
                new.other = util::read_u8(file);
                new.shndx = util::read_u16(file) as usize;
                new.value = util::read_u64(file);
                new.size  = util::read_u64(file);

                new
            }
        }
    }
}

pub mod object {
    use super::header::Header;
    use super::section::{self,Section};
    use super::sym::Sym;

    /// Represents a whole object file.
    pub struct Object {
        /// Name of the object file.
        name: String,
        /// Main ELF header.
        header: Header,
        /// Sections contained in the object file.
        sections: Vec<Section>,
        /// Symbols contained in the object file.
        symbols: Vec<Sym>,
    }

    /// Simple object methods.
    impl Object {
        /// Default object.
        pub fn empty() -> Self {
            Self {
                name:       "null".to_string(),
                header:     Header::empty(),
                sections:   vec![],
                symbols:    vec![],
            }
        }

    }

    /// Format methods.
    mod format {
        use std::fmt;
        use super::*;

        impl fmt::Display for Object {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let n_sections = self.sections.len();
                let n_symbols  = self.symbols.len();

                write!(f, "object:\n")?;
                write!(f, "- name : {}:\n", self.name)?;

                // print sections
                write!(f, "- sections : {}:\n", n_sections)?;
                let mut i = 0usize;
                while i < n_sections {
                    let section = &self.sections[i];
                    let name = self.section_name(i);
                    write!(f, "  - {} - {}: {:?}\n", i, name, section)?;
                    i += 1;
                }

                // print symbols
                write!(f, "- symbols : {}:\n", n_symbols)?;
                let mut i = 0usize;
                while i < n_symbols {
                    let symbol = &self.symbols[i];
                    let name = self.symbol_name(i);
                    write!(f, "  - {} - {}\n", i, name)?;
                    i += 1;
                }

                Ok(())
            }
        }
    }

    /// File IO methods.
    mod io {
        use std::fs::File;
        use std::io::{Seek,SeekFrom};
        use super::*;
        use super::super::util;

        impl Object {
            /// Generates an object from given file.
            ///
            /// TODO:
            /// - Add error checking / handling.
            pub fn extract(file: &mut File) -> Self {
                /* init default object */
                let mut new = Self::empty();

                /* extract ELF header */
                new.header = Header::extract(file);

                /* make sure its a valid file */
                assert!(new.header.valid());

                /* extract each section from the file */
                new.extract_sections(file);

                /* extract all symbols from symtab */
                new.extract_symbols(file);

                new
            }

            /// Extracts object file sections and populates
            /// the sections vector with them.
            fn extract_sections(&mut self, file: &mut File) {
                let off = self.header.shoff;
                let sz  = self.header.shentsize as u64;
                let num = self.header.shnum as u64;

                /* reset section vector */
                self.sections.clear();

                /* extract each section */
                let mut i = 0u64;
                while i < num {
                    let curr = off + sz * i;
                    file.seek(SeekFrom::Start(curr)).unwrap();
                    let section = Section::extract(file);
                    self.sections.push(section);
                    i += 1;
                }
            }

            /// Extracts symbol entries from the file and populates
            /// symbol vector with them.
            fn extract_symbols(&mut self, file: &mut File) {
                /* find symbol section */
                let mut i = 0;
                let count = self.sections.len();
                let symtab = loop {
                    let section = &self.sections[i];
                    if section.etype == section::Type::Symtab {
                        break section;
                    }
                    i += 1;
                    if i >= count {
                        panic!("symtab not found");
                    }
                };

                let off   = symtab.offset;      // offset into sym table
                let entsz = symtab.entsize;     // bytes size of symbol entry
                let num   = symtab.size/entsz;  // amount of symbols

                /* extract each symbol */
                self.symbols.clear();
                let mut i = 0u64;
                while i < num {
                    /* seek into next entry */
                    let curr = off + i * entsz;
                    file.seek(SeekFrom::Start(curr)).unwrap();
                    /* extract entry */
                    let sym = Sym::extract(file);
                    self.symbols.push(sym);
                    i += 1;
                }
            }

            /// Generates an object from given file name.
            pub fn from_file(filename: &str) -> Self {
                let mut file = File::open(filename).unwrap();
                let mut new = Self::extract(&mut file);
                new.name = filename.to_string();
                new
            }

            /// Returns the name of a section by the section index given.
            /// Needs to open the file to fetch the string.
            ///
            /// TODO:
            /// - Add section name cache, fetch from that first.
            /// - Error check for section index bounds.
            pub fn section_name(&self, ndx: usize) -> String {
                /* seek into string in file */
                let mut file = File::open(&self.name).unwrap();

                let section = &self.sections[ndx];      // the section we want
                let name = section.name;                // offset into name
                let tabndx = self.header.shstrndx;      // index for str-table
                let strtab = &self.sections[tabndx];
                let off = strtab.offset + name as u64;
                file.seek(SeekFrom::Start(off)).unwrap();

                /* read string untill null-byte */
                let mut s: Vec<u8> = vec![];
                let mut c: u8;
                loop {
                    c = util::read_u8(&mut file);
                    if c == b'\0' {
                        break;
                    }
                    s.push(c);
                }
                String::from_utf8(s).unwrap()
            }

            /// Returns the name of the symbol by the given index.
            pub fn symbol_name(&self, ndx: usize) -> String {
                /* seek into string in file */
                let mut file = File::open(&self.name).unwrap();

                /* find strtab for syms */
                let mut i = 0;
                let strtab = loop {
                    let section = &self.sections[i];
                    if section.etype == section::Type::Strtab {
                        break section;
                    }
                    i += 1;
                    if i >= self.sections.len() {
                        panic!("no strtab found");
                    }
                };

                let sym  = &self.symbols[ndx];
                let name = sym.name;
                let off  = strtab.offset + name as u64;
                file.seek(SeekFrom::Start(off)).unwrap();

                /* read string untill null-byte */
                let mut s: Vec<u8> = vec![];
                let mut c: u8;
                loop {
                    c = util::read_u8(&mut file);
                    if c == b'\0' {
                        break;
                    }
                    s.push(c);
                }
                String::from_utf8(s).unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::header::Header;
    use super::object::Object;

    #[test]
    fn magic() {
        let h = Header::from_file("/home/ed/repos/elf/samples/main.o");
        assert!(h.valid());
        let h = Header::from_file("/home/ed/repos/elf/samples/main.c");
        assert!(!h.valid());
    }

    #[test]
    fn section_name() {
        let o = Object::from_file("/home/ed/repos/elf/samples/main.o");
        assert_eq!(o.section_name(2), ".data");
    }

    #[test]
    fn display() {
        let o = Object::from_file("/home/ed/repos/elf/samples/main.o");
        println!("{}", o);
    }
}

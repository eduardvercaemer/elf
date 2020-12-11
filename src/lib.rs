mod util;

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

        impl fmt::Display for Type {
            /// Convert header type to string.
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s: &'static str = match self {
                    Self::Null => "unknown",
                    Self::Rel  => "relocatable",
                    Self::Exec => "executable",
                    Self::Dyn  => "shared object",
                    Self::Core => "core",
                };
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
        pub nameoff:    usize,      // 32-bits
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

        /// Extracted name string.
        pub name:       Option<String>,
    }

    impl Type {
        /// Default type.
        pub fn empty() -> Self {
            Self::Unhandled
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
    }

    /// Format methods.
    mod format {
        use std::fmt;
        use super::*;

        impl fmt::Display for Type {
            /// Convert a section type to string.
            /// **TODO: add rest of types.**
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s: &'static str = match self {
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
                };
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
}

/// Relevant to symbol entries.
mod sym {
    /// Posible symbol types.
    /// Obtained from the lower 4 bits of the info byte.
    #[derive(PartialEq)]
    pub enum Type {
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
    pub enum Bind {
        Local,
        Global,
        Weak,
        Unhandled,
    }

    /// Represents an individual entry in a symbol table.
    pub struct Sym {
        /// Index into the symbol string table.
        pub nameoff:    usize,      // 32-bits
        pub etype:      Type,       // \_ 8-bits
        pub bind:       Bind,       // /
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
    }

    /// Format methods.
    mod format {
        use std::fmt;
        use super::*;

        impl fmt::Display for Type {
            /// Convert our symbol type into a string.
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s: &'static str = match self {
                    Self::NoType    => "no type",
                    Self::Object    => "object",
                    Self::Func      => "function",
                    Self::Section   => "section",
                    Self::File      => "file",
                    Self::Common    => "common",
                    Self::TLS       => "tls",
                    Self::Num       => "num",
                    Self::Unhandled => "unhandled",
                };
                write!(f, "{}", s)
            }
        }

        impl fmt::Display for Bind {
            /// Convert our symbol binding into a string.
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s: &'static str = match self {
                    Self::Local     => "local",
                    Self::Global    => "global",
                    Self::Weak      => "weak",
                    Self::Unhandled => "unhandled",
                };
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
}

pub mod object {
    use super::header::Header;
    use super::section::{self,Section};
    use super::sym::{self,Sym};

    /// Represents a whole object file.
    pub struct Object {
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
                header:     Header::empty(),
                sections:   vec![],
                symbols:    vec![],
            }
        }

    }

    /// Format methods.
    mod format {
        use super::*;

        /// Displaying methods.
        impl Object {
            pub fn print(&self) {
                println!("{0:^80}\n", "==========   Object   ==========");

                println!(" <> SECTIONS");
                println!("  {0: <10} {1: <10} {2: <30}",
                    "offset", "type", "name");

                for s in &self.sections {
                    let name = s.name.as_ref().unwrap();
                    let off  = s.offset;
                    let t    = s.etype.to_string();

                    println!("  {0:#010x} {1: <10} {2: <30}",
                        off, t, name);
                }

                println!("\n <> SYMBOLS");
                println!("  {0: <10} {1: <10} {2: <10} {3: <30}",
                    "value", "bind", "type", "name");

                for s in &self.symbols {
                    let name = s.name.as_ref().unwrap();
                    let val  = &s.value;
                    let bind = s.bind.to_string();
                    let t    = s.etype.to_string();

                    println!("  {0:#010x} {1: <10} {2: <10} {3: <30}",
                        val, bind, t, name);
                }
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
            /// Generates a complete object file representation
            /// from the given file name.
            pub fn from_file(filename: &str) -> Self {
                let mut file = File::open(filename).unwrap();
                Self::extract(&mut file)
            }

            /// Extracts a complete `Object` from given file.
            ///
            /// Will use the given file, to extract all the information it
            /// needs.
            /// - First the main ELF header.
            /// - Then all sections.
            /// - Then all symbols.
            /// - Then all the names for these.
            fn extract(file: &mut File) -> Self {
                /* init default object */
                let mut new = Self::empty();

                /* extract properties from file */
                new.extract_header(file);
                assert!(new.header.valid());
                new.extract_sections(file);
                new.extract_symbols(file);
                new.extract_section_names(file);
                new.extract_symbol_names(file);

                new
            }

            /// Populates the object's ELf header with the info
            /// extracted from the given file.
            fn extract_header(&mut self, file: &mut File) {
                /* go to beginning of file */
                file.seek(SeekFrom::Start(0)).unwrap();
                /* extract header */
                self.header = Header::extract(file);
            }

            /// Populates the object's section vector with the info
            /// extracted from the given file.
            ///
            /// Will extract sections based on the values of
            /// `self.header`.
            ///
            /// - **Requires a valid ELF header to have been loaded first.**
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

            /// Populates the object's symbols vector with the info
            /// extracted from the given file.
            ///
            /// Will extract symbols based on the values of the sections
            /// vector `self.sections`.
            ///
            /// - **Requires a valid ELF header to have been loaded first.**
            /// - **Requires a valid sections vector to have been loaded first.**
            fn extract_symbols(&mut self, file: &mut File) {
                /* find symtab section */
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

            /// Will update all the sections in `self.sections` by extracting
            /// their name from the given file.
            ///
            /// - **Requires a valid ELF header to have been loaded first.**
            /// - **Requires a valid sections vector to have been loaded first.**
            fn extract_section_names(&mut self, file: &mut File) {
                let num = self.sections.len();
                let mut i = 0;
                /* extract each name */
                while i < num {
                    let name = self.extract_section_name(file, i);
                    self.sections[i].name = Some(name);
                    i += 1;
                }
            }

            /// Will update all the symbols in `self.symbols` by extracting
            /// their name from the given file.
            ///
            /// - **Requires a valid ELF header to have been loaded first.**
            /// - **Requires a valid sections vector to have been loaded first.**
            /// - **Requires a valid symbols vector to have been loaded first.**
            fn extract_symbol_names(&mut self, file: &mut File) {
                let num = self.symbols.len();
                let mut i = 0;
                /* extract each name */
                while i < num {
                    let name = self.extract_symbol_name(file, i);
                    self.symbols[i].name = Some(name);
                    i += 1;
                }
            }


            /// Extracts the name of a section by the section index given.
            ///
            /// **Requires all sections to be loaded**
            fn extract_section_name(&self, file: &mut File, ndx: usize) -> String {
                let section = &self.sections[ndx];        // the section we want
                let nameoff = section.nameoff;            // offset into name
                let tabndx  = self.header.shstrndx;       // index for str-table
                let strtab  = &self.sections[tabndx];
                let off = strtab.offset + nameoff as u64; // final offset

                /* seek into string */
                file.seek(SeekFrom::Start(off)).unwrap();

                /* read string untill null-byte */
                let mut s: Vec<u8> = vec![];
                let mut c: u8;
                loop {
                    c = util::read_u8(file);
                    if c == b'\0' {
                        break;
                    }
                    s.push(c);
                }

                String::from_utf8(s).unwrap()
            }

            /// Extracts the name of a symbol by the index given.
            ///
            /// **Requires all sections to be loaded**
            /// **Requires all symbols to be loaded**
            fn extract_symbol_name(&self, file: &mut File, ndx: usize) -> String {
                let sym     = &self.symbols[ndx];       // the symbol we want

                /* section symbols get their name from the section
                 * they represent
                 */
                if sym.etype == sym::Type::Section {
                    /* for section symbols, we use the shndx member
                     * to get the corresponding name
                     */
                    let ndx = sym.shndx;
                    return self.sections[ndx].name.as_ref().unwrap().clone();
                }

                /* otherwise the name comes from the file's symbol
                 * string table
                 */
                let mut i = 0;
                let tabndx = loop {
                        let section = &self.sections[i];
                        if section.etype == section::Type::Strtab {
                            break i;
                        }
                        i += 1;
                        if i >= self.sections.len() {
                            panic!("no strtab found");
                        }
                };
                let strtab = &self.sections[tabndx];

                /* seek into string in file */
                let nameoff = sym.nameoff;
                let off = strtab.offset + nameoff as u64;
                file.seek(SeekFrom::Start(off)).unwrap();

                /* read string untill null-byte */
                let mut s: Vec<u8> = vec![];
                let mut c: u8;
                loop {
                    c = util::read_u8(file);
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
    
    /*
     * TODO:
     */
}

mod util;
mod header;
mod segment;
mod section;
mod sym;

pub mod object {
    use super::header::Header;
    use super::section::Section;
    use super::segment::Segment;
    use super::sym::Sym;

    /// Represents a whole object file.
    pub struct Object {
        /// Main ELF header.
        header: Header,
        /// Sections contained in the object file.
        sections: Vec<Section>,
        /// Segments contained in the object file.
        segments: Vec<Segment>,
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
                segments:   vec![],
            }
        }

    }

    /// Format methods.
    mod format {
        use super::*;

        /// Displaying methods.
        impl Object {
            pub fn print(&self) {
                println!("{0:^80}",
                    "==========   ELF Object   ==========");

                println!("\n <> HEADER\n");
                let t = self.header.type_str();
                println!("  {0: >10} : {1: <10}",
                    "type", t);
                let entry = self.header.entry;
                println!("  {0: >10} : {1: <10}",
                    "entry", format!("{0:#010X}", entry));

                println!("\n <> SEGMENTS\n");
                println!("  {0: <10} {1: <10} {2: <30} {3: <10}\n",
                    "vaddr", "paddr", "type", "align");

                for s in &self.segments {
                    let vaddr = s.vaddr;
                    let paddr = s.vaddr;
                    let t     = s.type_str();
                    let align = s.align;

                    println!("  {0:#010x} {1:#010x} {2: <30} {3: <10}",
                        vaddr, paddr, t, align);
                }


                println!("\n <> SECTIONS\n");
                println!("  {0: <10} {1: <10} {2: <30}\n",
                    "offset", "type", "name");

                for s in &self.sections {
                    let name = s.name.as_ref().unwrap();
                    let off  = s.offset;
                    let t    = s.type_str();

                    println!("  {0:#010x} {1: <10} {2: <30}",
                        off, t, name);
                }

                println!("\n <> SYMBOLS\n");
                println!("  {0: <10} {1: <10} {2: <10} {3: <30}\n",
                    "value", "bind", "type", "name");

                for s in &self.symbols {
                    let name = s.name.as_ref().unwrap();
                    let val  = &s.value;
                    let bind = s.bind_str();
                    let t    = s.type_str();

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
                new.extract_segments(file);
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

            /// Populates the object's segment vector with the info
            /// extracted from the given file.
            ///
            /// Will extract segments based on the values of
            /// `self.header`.
            ///
            /// - **Requires a valid ELF header to have been loaded first.**
            fn extract_segments(&mut self, file: &mut File) {
                let off = self.header.phoff;
                let sz  = self.header.phentsize as u64;
                let num = self.header.phnum as u64;

                /* reset segment vector */
                self.segments.clear();

                /* extract each segment */
                let mut i = 0u64;
                while i < num {
                    let curr = off + sz * i;
                    file.seek(SeekFrom::Start(curr)).unwrap();
                    let segment = Segment::extract(file);
                    self.segments.push(segment);
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
                    if section.is_symtab() {
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
                let sym = &self.symbols[ndx];       // the symbol we want

                /* section symbols get their name from the section
                 * they represent
                 */
                if sym.is_section() {
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
                        if section.is_strtab() {
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

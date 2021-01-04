extern crate elf;

use elf::object::Object;

pub fn main() {
    let obj = Object::from_file("samples/main.o");
    obj.print();
}

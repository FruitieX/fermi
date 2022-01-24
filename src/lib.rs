pub mod prelude {
    pub use crate::*;
}

mod root;
mod selector;

pub use atoms::*;
pub use hooks::*;
pub use root::*;
pub use selector::*;

mod atoms {
    mod atom;
    mod atomfamily;
    mod atomref;
    mod selector;
    mod selectorfamily;

    pub use atom::*;
    pub use atomfamily::*;
    pub use atomref::*;
    pub use selector::*;
    pub use selectorfamily::*;
}

pub mod hooks {
    mod atom_ref;
    mod atom_root;
    mod init_atom_root;
    mod read;
    mod set;
    pub use atom_ref::*;
    pub use atom_root::*;
    pub use init_atom_root::*;
    pub use read::*;
    pub use set::*;
}

pub trait Readable<V> {
    fn read(&self, root: AtomRoot) -> Option<V>;
    fn init(&self) -> V;
    fn unique_id(&self) -> AtomId;
}

pub trait Writable<V>: Readable<V> {
    fn write(&self, root: AtomRoot, value: V);
}

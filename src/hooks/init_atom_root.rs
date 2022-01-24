use crate::AtomRoot;
use dioxus::core::ScopeState;
use std::rc::Rc;

// Initializes the atom root and retuns it;
pub fn use_init_atom_root(cx: &ScopeState) -> &Rc<AtomRoot> {
    cx.use_hook(|_| match cx.consume_context::<AtomRoot>() {
        Some(ctx) => ctx,
        None => cx.provide_context(AtomRoot::new(cx.schedule_update_any())),
    })
}

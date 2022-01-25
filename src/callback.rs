use std::{future::Future, rc::Rc};

use dioxus::prelude::*;

use crate::{Atom, AtomRoot, Readable, Writable};

#[derive(Clone)]
pub struct CallbackApi {
    root: Rc<AtomRoot>,
}

impl CallbackApi {
    // get the current value of the atom
    pub fn get<V>(&self, atom: impl Readable<V>) -> &V {
        todo!()
    }

    // get the current value of the atom in its RC container
    pub fn get_rc<V>(&self, atom: impl Readable<V>) -> &Rc<V> {
        todo!()
    }

    // set the current value of the atom
    pub fn set<V>(&self, atom: impl Writable<V>, value: V) {
        todo!()
    }
}

pub fn use_atom_context(cx: &ScopeState) -> &CallbackApi {
    todo!()
}

macro_rules! use_callback {
    (&$cx:ident, [$($cap:ident),*],  move || $body:expr) => {
        move || {
            $(
                #[allow(unused_mut)]
                let mut $cap = $cap.to_owned();
            )*
            $cx.spawn($body);
        }
    };
}

#[macro_export]
macro_rules! to_owned {
    ($($es:ident),+) => {$(
        #[allow(unused_mut)]
        let mut $es = $es.to_owned();
    )*}
}

#[test]
fn test_fermi_callback() {
    static THING: Atom<i32> = |_| 10;

    fn app(cx: Scope<()>) -> Element {
        let data = use_ref(&cx, Vec::<String>::new);
        let api = use_atom_context(&cx);

        let enable_thing = move || {
            let data = data.to_owned();
            let api = api.to_owned();
            cx.spawn(async move {
                match api.get(THING) {
                    10 => api.set(THING, 100),
                    20 => api.set(THING, 200),
                    _ => {}
                }
                data.write().push("asd".to_string());
            });
        };

        let enable_bling = use_callback!(&cx, [data, api], move || async move {
            match api.get(THING) {
                10 => api.set(THING, 100),
                20 => api.set(THING, 200),
                _ => {}
            }
            data.write().push("asd".to_string());
        });

        cx.render(rsx! {
            button {
                onclick: move |_| enable_thing(),
                onclick: move |_| enable_bling(),
                "Enable Thing"
            }
        })
    }
}

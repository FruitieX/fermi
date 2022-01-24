# Fermi: An atom-based global state management solution for Dioxus

Fermi is a global state management solution for Dioxus that's as easy as `use_state`.

Inspired by atom-based state management solutions, all state in Fermi starts as an `atom`:

```rust
static NAME: Atom<String> = |_| "Dioxus".to_string();
```

From anywhere in our app, we can read our the value of our atom:

```rust
fn NameCard(cx: Scope) -> Element {      
    let name = use_read(&cx, NAME);
    cx.render(rsx!{ h1 { "Hello, {name}"} })
}
```

We can also set the value of our atom, also from anywhere in our app:

```rust
fn NameCard(cx: Scope) -> Element {      
    let set_name = use_set(&cx, NAME);
    cx.render(rsx!{
        button {
            onclick: move |_| set_name("Fermi".to_string()),
            "Set name to fermi"
        }
    })
}
```

It's that simple!

## Installation
Fermi is currently under construction, so you have to use the `master` branch to get started.

```rust
[depdencies]
fermi = { git = "https://github.com/dioxuslabs/fermi" }
```


## Running examples

The examples here use Dioxus Desktop to showcase their functionality. To run an example, use
```
$ cargo run --example EXAMPLE
```

## Features

Broadly our feature set to required to be released includes:
- [x] Support for Atoms
- [x] Support for AtomRef (for values that aren't clone)
- [ ] Support for Atom Families
- [ ] Support for memoized Selectors
- [ ] Support for memoized SelectorFamilies
- [ ] Support for UseFermiCallback for access to fermi from async 

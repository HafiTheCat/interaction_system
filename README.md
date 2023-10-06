# 👋Hi
I got inspired by the typical point and click adventure structure. Which makes operations like `Inspect` or `Interact` available on **Elements** (Hotspots) of a scene.

This repo contains an experimental skeleton for defining interactions on Elements.

# 📔 Usage
Rust Compiler required.
`cargo run`

Some tests are already present.
To run the available tests `cargo test`

## Adding a new Element
To add a new element create a new struct that implements the `TElement` trait.

## Making and Element Interactable
Implement the respective Interaction trait e.g. `Interactable`, `Inspectable`.

## Adding a new Interaction
1. Add an entry to the `Interaction` enum
2. create a trait for the Interaction like below

```rust
pub trait Interaction_name {
    fn on_interaction_verb(&self) -> Box<dyn Action>;
}
```

## Adding a new Action
To add a new Action create a new struct that implements the `Action` trait.
If needed the `then` function of the `Action` trait can also be implemented.

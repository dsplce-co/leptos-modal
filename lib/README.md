> We're dsplce.co, check out our work on our website: [dsplce.co](https://dsplce.co) 🖤

# leptos-modal

[![Leptos](https://img.shields.io/badge/Leptos-0.8-EF3939?style=for-the-badge&logo=leptos&logoColor=white)](https://leptos.dev/)
[![crates.io Downloads](https://img.shields.io/crates/d/leptos-modal?style=for-the-badge&color=%23FF0346)](https://crates.io/crates/leptos-modal)
[![crates.io Size](https://img.shields.io/crates/size/leptos-modal?style=for-the-badge)](https://crates.io/crates/leptos-modal)
[![License](https://img.shields.io/crates/l/leptos-modal.svg?style=for-the-badge)](https://crates.io/crates/leptos-modal)
[![crates.io](https://img.shields.io/crates/v/leptos-modal?style=for-the-badge&color=%230F80C1)](https://crates.io/crates/leptos-modal)

🌀 Modal composable for [Leptos](https://leptos.dev/) — minimal, type-safe, zero CSS to fight.

`leptos-modal` is the modal layer every app ends up writing by hand: you wrap the root once, define a modal as a plain component, and open it from anywhere — no mounting it deep in the tree, no prop-drilling a setter down five levels, no z-index wars. The data you pass in is checked by the compiler, not by your users at runtime.

## 🖤 Features

- **Type-safe by construction** — pass the wrong data to a modal and `cargo` stops _you_, not the person clicking the button. Generics carry the input and context types end to end.
- **Context _and_ input, kept separate** — the constant stuff (like a user-delete callback) goes in once on registration; the dynamic stuff (which user) goes in on open. Two channels, no `Option` soup.
- **Close from wherever you are** — call the `close()` handed to your modal, or `modal.close()` from the controller. No setter threaded down the component tree.
- **ARIA-compliant** — `role="dialog"`, `aria-modal`, the lot, out of the box.
- **Esc closes it** — keyboard handling you didn't have to wire up.
- **A `#[modal]` proc macro** — write a normal component, get a registerable modal.
- **Zero external CSS dependencies** — nothing to import, nothing to override.
- **Portal-style rendering** — one place modals render, always exactly one on screen (why would you ever want to show two at once? 🤨), with a z-index that _always wins_.



![z-index always wins](./assets/harvey-specter.gif)

---

## Table of Contents

- [🖤 Features](#-features)
- [📦 Installation](#-installation)
- [🧪 Usage](#-usage)
  - [Set up the modal collector](#set-up-the-modal-collector)
  - [Create a modal component](#create-a-modal-component)
  - [Open and close the modal](#open-and-close-the-modal)
- [📐 API Reference](#-api-reference)
  - [`ModalCollector`](#modalcollector)
  - [`#[modal]`](#modal)
  - [`use_modal!`](#use_modal)
  - [Closing a modal](#closing-a-modal)
  - [Defining a modal](#defining-a-modal)
  - [Built-in behaviour](#built-in-behaviour)
- [🛠️ Requirements](#%EF%B8%8F-requirements)
- [📁 Repo & Contributions](#-repo--contributions)
- [📄 License](#-license)

⸻

## 📦 Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
leptos-modal = "0.3"
```

Or from the terminal:

```bash
cargo add leptos-modal
```

⸻

## 🧪 Usage

### Set up the modal collector

Wrap your app with `ModalCollector` to enable modal rendering. Think of it like a provider in React — it lets any of its descendants instantiate and use modals without each one having to mount them.

```rust
use leptos::prelude::*;
use leptos_modal::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <ModalCollector>
            <MainContent />
        </ModalCollector>
    }
}
```

### Create a modal component

Say your app has a user list, and you want to delete a user from it. A confirmation dialog would come in handy.

A `leptos-modal` modal is a component with this signature:

```rust
#[modal]
pub fn ConfirmationModal(input: Input, ctx: Context, close: fn()) -> impl IntoView;
```

Where:

- `Input` is dynamic data you don't know until the modal opens (here, the user to delete). Must satisfy `Clone + Send + Sync + 'static`.
- `Context` is something constant, passed in once on registration and not changeable after (here, the function that does the deleting). Must satisfy `Clone + Copy + Send + Sync + 'static`.
- `close` is the handle that dismisses the modal — call it from inside.

Here's the confirmation dialog:

```rust
use leptos_modal::prelude::*;

#[modal]
pub fn ConfirmationModal(user: User, ctx: Callback<String>, close: fn()) -> impl IntoView {
    view! {
        <div class="confirmation-modal">
            <h2>"Confirm Action"</h2>
            <p>{move || format!("Are you sure you want to delete {}?", user.name)}</p>

            <div class="confirmation-modal__actions">
                <button on:click=move |_| close()>"Cancel"</button>
                <button on:click=move |_| {
                    ctx.run(user.id.clone());
                    close();
                }>"Confirm"</button>
            </div>
        </div>
    }
}
```

### Open and close the modal

Register it with the `use_modal!` macro, then `open` it with the input:

```rust
#[derive(Clone)]
struct User {
    id: String,
    name: String,
}

#[component]
fn UsersView(users: Vec<User>) -> impl IntoView {
    let delete_user = Callback::new(move |id: String| {
        // Deletion logic
    });

    // Registers the modal (the context is the delete callback)
    let modal = use_modal!(ConfirmationModal, delete_user);

    let on_delete = move |user: User| {
        modal.open(user);
    };

    view! {
        // ❗ Notice the `ConfirmationModal` is not mounted directly
        // anywhere — it is the `ModalCollector`'s job to render modals
        <ul>
            {
                move || users.iter().map(|user| view! {
                    <li>
                        {user.name.clone()}
                        <button
                            on:click={
                                let user = user.clone();

                                move |_| {
                                    on_delete(user.clone());
                                }
                            }
                        >"Delete"</button>
                    </li>
                }).collect::<Vec<_>>()
            }
        </ul>
    }
}
```

Closing happens from inside the modal via the injected `close()` (see the `"Cancel"` button above), from the controller with `modal.close()`, or simply by hitting `Esc`.

⸻

## 📐 API Reference

### `ModalCollector`

Singleton component that manages modal state and rendering. We recommend it wraps your app root.

### `#[modal]`

Proc macro that turns a normal component into one the `ModalCollector` can render. Your function keeps the `(input, ctx, close)` signature; the macro does the wiring.

### `use_modal!`

Registers a modal and returns a typed controller:

```rust
// Without context
let modal = use_modal!(ModalComponent);

// With context
let modal = use_modal!(ModalComponent, context);
```

The controller exposes:

- `open(input)` — opens the modal with the provided input.
- `close()` — closes the modal.

### Closing a modal

There's no single global close — closing is always tied to a modal you've registered. You have three ways to dismiss one:

- Inside the modal component, call the injected `close()` (the third argument of the signature).
- From the controller returned by `use_modal!`, call `modal.close()`.
- Let the user press `Esc` — the `ModalCollector` handles that for you.

### Defining a modal

#### With both context and input

```rust
#[modal]
pub fn ModalComponent(input: Input, ctx: Context, close: fn()) -> impl IntoView {
    view! {
        // ...
    }
}

let modal = use_modal!(ModalComponent, context);
modal.open(input)
```

#### Skipping context

```rust
#[modal]
pub fn ModalComponent(input: Input, ctx: (), close: fn()) -> impl IntoView {
    view! {
        // ...
    }
}

let modal = use_modal!(ModalComponent);
modal.open(input)
```

#### Skipping input

```rust
#[modal]
pub fn ModalComponent(input: (), ctx: Context, close: fn()) -> impl IntoView {
    view! {
        // ...
    }
}

let modal = use_modal!(ModalComponent, context);
modal.open(())
```

### Built-in behaviour

- **Accessibility** — proper ARIA attributes (`role="dialog"`, `aria-modal`).
- **Keyboard navigation** — `Esc` closes the modal out of the box.
- **Portal rendering** — modals render in one place, and there's always exactly one on screen at a time (why would you want more than one modal up at once? 🤨).
- **Overlay** — semi-transparent backdrop with proper positioning.
- **Responsive** — full viewport coverage with centered content.

⸻

## 🛠️ Requirements

- **Rust 2024 edition** — the crate targets the 2024 edition.
- **Leptos 0.8** — built and tested against the 0.8 line.

⸻

## 📁 Repo & Contributions

🛠️ **Repo**: [https://github.com/dsplce-co/leptos-modal](https://github.com/dsplce-co/leptos-modal)<br>
📦 **Crate**: [https://crates.io/crates/leptos-modal](https://crates.io/crates/leptos-modal)

Contributions, issues, ideas? Hit us up 🖤

⸻

## 📄 License

MIT or Apache-2.0, at your option.

> We're dsplce.co, check out our work on [github.com/dsplce-co](https://github.com/dsplce-co) 🖤

# leptos-modal

🌀 **Modal composable for Leptos** — A minimal and type-safe framework for modals in [Leptos](https://leptos.dev/) applications.

---

## 🖤 Features

✅ Type-safe modal system with generics<br>
✅ Pass additional context to your modals<br>
✅ Close your modals from anywhere with a global fn<br>
✅ ARIA-compliant<br>
✅ Esc key handling<br>
✅ Portal-style rendering with proper z-index<br>
✅ Proc macro for modal component creation<br>
✅ Zero external CSS dependencies<br>

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos-modal = "0.3"
```

This crate requires Rust 2024 edition and is compatible with Leptos 0.8.

---

## 🧪 Usage

### 1. Set up modal collector

Wrap your app with `ModalCollector` to enable modal rendering. This works similar to a provider in React, in that it allows modals to be instantiated and used from the level of any of its descendants.

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

### 2. Create modal component

Imagine in your application there is a user list view, and you want to add the functionality to delete a user to it. You decide a confirmation dialog would come in handy.

In `leptos-modal`, your modal component needs to adhere to the following signature:

```rust
#[modal]
pub fn ConfirmationModal(input: Input, ctx: Context, close: fn()) -> impl IntoView;
```

Where:
- `Input` is dynamic data typically not known until the modal's opening is triggered (in our example it would be the user to delete). Should satisfy `Clone + Send + Sync + 'static`
- `Context` is something constant, passed to the modal on registration and thus not changeable (eg. a function responsible for user deletion). Should satisfy `Clone + Copy + Send + Sync + 'static`

Let's implement the confirmation dialog:

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

### 3. Use the modal

Now that you've defined the confirmation modal, let's call it using the `use_modal!` macro:

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

    // Registers the modal
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

---

## 📐 API Reference

### `ModalCollector`

Singleton component that manages modal state and rendering. We recommend that it wraps your app root.

### `#[modal]`

Proc macro that helps the `ModalCollector` render your modals.

### `use_modal!`

Creates a typed modal controller:

```rust
// Without context
let modal = use_modal!(ModalComponent);

// With context
let modal = use_modal!(ModalComponent, context);
```

Returns a modal struct with the methods:
- `open(args)` - Opens the modal with provided arguments
- `close()` - Closes the modal

### `close`

Close the modal from any component (that is a descendant of `ModalCollector`) in your application:

```rust
leptos_modal::close();
```

### Defining a modal

#### With both context and input:

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

#### Skipping context:

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

#### Skipping input:

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

### Modal Features

- **Accessibility**: Proper ARIA attributes (`role="dialog"`, `aria-modal`)
- **Keyboard Navigation**: Esc key closes modal out of the box
- **Portal Rendering**: Modals have a single place to render, and there is always one modal visible at a time (why would you want to show more than one modal at a time? 🤨)
- **Overlay**: Semi-transparent backdrop with proper positioning
- **Responsive**: Full viewport coverage with centered content

---

## 📁 Repo & Contributions

📦 Crate: [crates.io/crates/leptos-modal](https://crates.io/crates/leptos-modal)<br/>
🛠️ Repo: [github.com/dsplce-co/leptos-modal](https://github.com/dsplce-co/leptos-modal)<br/>

Contributions, issues, ideas? Hit us up 🖤

---

## 🔒 License

MIT or Apache-2.0, at your option.

---

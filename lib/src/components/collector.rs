use leptos::{ev::keyup, prelude::*};
use leptos_use::{use_document, use_event_listener};

pub type ModalView = ChildrenFn;

#[component]
pub fn ModalCollector(children: Children) -> impl IntoView {
    let modal = RwSignal::new(None::<ModalView>);

    provide_context(modal);

    let _ = use_event_listener(use_document(), keyup, move |event| {
        if event.key() == "Escape" {
            modal.set(None);
        }
    });

    view! {
        <div class="leptos_modal-collector" style:display="contents">
            {children()}

            <div
                role="dialog"
                style:position="relative"
                style:z-index="2147483647"
                aria-modal={move || modal.read().is_some().to_string()}
                aria-labelledby="modal-title"
            >
                {move || modal.read().as_ref().map(|_| view! {
                    <div
                        aria-hidden="true"
                        id="leptos_modal-overlay"
                        style:position="fixed"
                        style:top="0"
                        style:right="0"
                        style:bottom="0"
                        style:left="0"
                        style:background="rgba(0, 0, 0, .25)"
                    />
                }.into_any())}

                {move || modal.read().as_ref().map(|view_fn| {
                    let render: &ChildrenFn = &*view_fn;

                    view! {
                        <div
                            id="leptos_modal-node"
                            style:position="fixed"
                            style:top="0"
                            style:right="0"
                            style:bottom="0"
                            style:left="0"
                            style:z-index="2147483646"
                            style:width="100vw"
                            style:overflow-y="auto"
                        >

                            <div
                                style:padding="1rem"
                                style:justify-content="center"
                                style:align-items="center"
                                style:min-height="100%"
                                style:display="flex"
                            >
                                {render()}
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

use leptos::prelude::*;
use std::cell::Cell;
use std::marker::PhantomData;

use crate::components::ModalView;

thread_local! {
    pub static ELEMENT: Cell<Option<AnyView>> = Cell::new(None);
}

#[derive(Clone)]
pub struct Modal<Arg, ModalComponent, Context>
where
    ModalComponent: Fn(Arg, Context, fn()) -> AnyView + Clone + Copy + Send + Sync,
    Context: Clone + Copy + Send + Sync,
{
    element: ModalComponent,
    context: Context,
    _marker: PhantomData<Arg>,
}

impl<Arg, ModalComponent, Context> Copy for Modal<Arg, ModalComponent, Context>
where
    Arg: Clone,
    ModalComponent: Fn(Arg, Context, fn()) -> AnyView + Clone + Copy + Send + Sync,
    Context: Clone + Copy + Send + Sync,
{
}

impl<Arg, ModalComponent, Context> Modal<Arg, ModalComponent, Context>
where
    Arg: Clone,
    ModalComponent: Fn(Arg, Context, fn()) -> AnyView + Clone + Copy + Send + Sync,
    Context: Clone + Copy + Send + Sync,
{
    pub fn new(element: ModalComponent, context: Context) -> Self {
        Self {
            element,
            context,
            _marker: PhantomData,
        }
    }

    pub fn open(&self, arg: Arg) {
        let modal =
            use_context::<RwSignal<Option<ModalView>>>().expect("No `modal` signal provided");

        let close_fn = move || {
            let modal =
                use_context::<RwSignal<Option<ModalView>>>().expect("No `modal` signal provided");
            modal.set(None);
            ELEMENT.set(None);
        };

        let html = (self.element)(arg, self.context, close_fn);
        ELEMENT.set(Some(html));

        modal.set(Some(std::sync::Arc::new(move || {
            ELEMENT
                .with(|el| el.take())
                .unwrap_or_else(|| view! {}.into_any())
        })));
    }

    pub fn close(&self) {
        let modal =
            use_context::<RwSignal<Option<ModalView>>>().expect("No `modal` signal provided");
        modal.set(None);
        ELEMENT.set(None);
    }
}

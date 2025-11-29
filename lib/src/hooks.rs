use leptos::prelude::*;

use crate::abstracts::Modal;

pub fn _use_modal<ModalComponent, Input, Context>(
    element: ModalComponent,
    context: Context,
) -> Modal<Input, ModalComponent, Context>
where
    ModalComponent: Fn(Input, Context, fn()) -> AnyView + Clone + Copy + Send + Sync + 'static,
    Input: Clone + Send + Sync + 'static,
    Context: Clone + Copy + Send + Sync + 'static,
{
    Modal::new(element, context)
}

#[macro_export]
macro_rules! use_modal {
    ($component:expr, $ctx:expr) => {
        $crate::_use_modal($component, $ctx)
    };
    ($component:expr) => {
        $crate::_use_modal($component, ())
    };
}

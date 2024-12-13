use codee::string::FromToStringCodec;
use leptos::{component, logging::log, view, IntoView};
use leptos_use::storage::use_session_storage;
use reactive_graph::traits::{Get, Update};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BananaState {
    pub name: String,
}

impl Default for BananaState {
    fn default() -> Self {
        Self {
            name: "Bananas".to_string(),
        }
    }
}

#[component]
pub fn Demo() -> impl IntoView {
    let (state, set_state, _) = use_session_storage::<String, FromToStringCodec>("banana-state");

    let click = move |_| set_state.update(|s| *s = "Bananas".to_string());

    log!("state: {:?}", state.get());

    view! { <button on:click=click>"Set banana state"</button> }
}

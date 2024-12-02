use std::collections::VecDeque;

use crate::error_template::{AppError, ErrorTemplate};

use crate::server_fn::codec::StreamingText;
use futures::{stream, StreamExt};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use logging::log;
use server_fn::codec::TextStream;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=StreamExample/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn StreamExample() -> impl IntoView {
    let (is_streaming, set_is_streaming) = create_signal(false);

    let start_stream = move |_| {
        set_is_streaming.set(true);
    };

    let (data, set_data) = create_signal(vec![]);

    create_effect(move |_| {
        if is_streaming.get() {
            spawn_local(async move {
                let stream_future = async {
                    let mut stream = stream_data().await.unwrap().into_inner();

                    while let Some(item) = stream.next().await {
                        let result = item.unwrap();
                        log!("RESULT: {:?}", result);
                        set_data.update(|data| data.push(result));
                    }
                };

                stream_future.await;
            });
        }
    });

    view! {
        <div>
            <button on:click=start_stream>
                "Start Stream"
            </button>

            {move || if is_streaming.get() {
                view! {
                    <div>
                        <p>"Receiving data from server..."</p>
                        <ul>
                            <For each=move || data.get() key=|item| item.to_string() children=|item| view! {
                                <li>{item}</li>
                            }/>
                        </ul>
                    </div>
                }
            } else {
                view! {
                    <div>"Press button to start stream"</div>
                }
            }}
        </div>
    }
}

#[server(output = StreamingText)]
pub async fn stream_data() -> Result<TextStream, ServerFnError> {
    let vec = VecDeque::from(vec![1, 2, 3]);
    let result = stream::unfold(vec, |mut vec| async move {
        if let Some(item) = vec.pop_front() {
            log!("ITEM: {:?}", item);
            Some((Ok(item.to_string()), vec))
        } else {
            None
        }
    });

    Ok(TextStream::new(result.map(|result| {
        Ok(result.unwrap_or_else(|_: ServerFnError| "".to_string()))
    })))
}

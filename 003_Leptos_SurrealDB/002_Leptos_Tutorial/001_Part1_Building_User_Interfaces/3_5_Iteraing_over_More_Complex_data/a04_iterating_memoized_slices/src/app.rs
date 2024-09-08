use leptos::{
    component, create_memo, create_signal, logging, view, For, IntoView, SignalGet, SignalUpdate,
    SignalWith,
};

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn App() -> impl IntoView {
    // start with a set of three rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);
    view! {
            // when we click, update each row,
            // doubling its value
            <button on:click=move |_| {
                set_data.update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
                // log the new value of the signal
                logging::log!("{:?}", data.get());
            }>
                "Update Values"
            </button>
            // iterate over the rows and display each value

    <For
        each=move || data().into_iter().enumerate()
        key=|(_, state)| state.key.clone()
        children=move |(index, _)| {
            let value = create_memo(move |_| {
                data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
            });
            view! {
                <p>{value}</p>
            }
        }
    />
        }
}

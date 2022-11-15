use crate::apis::*;
use yew::prelude::*;
use yew::suspense::*;

#[function_component]
pub fn TodoList() -> HtmlResult {
    let future_users = use_future(|| async {
        let config = configuration::Configuration::default();
        let result = default_api::get_all_users(&config).await;
        result
    })?;

    let users = future_users.as_ref().unwrap();

    Ok(html! {
        <div>
            {users.iter().map(|item| {
                html! {<div></div>}
            }).collect::<Html>()}

            <form onsubmit={|_e| { web_sys::console::log_1(&format!("hello").into()) }}>
                <input />
            </form>
        </div>
    })
}

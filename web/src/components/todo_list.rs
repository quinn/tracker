use yew::prelude::*;
use yew::suspense::*;
use crate::apis::*;

#[function_component]
pub fn TodoList() -> Html {
    let future_users = use_future(|| async {
        let config = configuration::Configuration::default();
        let result = default_api::get_all_users(&config).await;
        result
    });

    let _users = future_users.unwrap();

    html! {
        <div>
            {"list of users goes here"}
        </div>
    }
}

use yew::prelude::*;
use crate::apis::*;

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;	
            counter.set(value);
        }
    };

    use_effect_with_deps(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let mut config = configuration::Configuration::default();
//            config.base_path = "/api".to_owned();
            let _thing = default_api::get_all_users(&config).await.unwrap();
        });
    }, ());

    html! {
        <div class="bg-neutral-900 text-lime-200">
            <div class="root">
                <div class="border-r border-neutral-700 p-3" {onclick}>
                    {"Sidebar"}
                </div>

                <div>
                    <div class="border-b border-neutral-700 p-3">
                        <h1 class="text-3xwl font-bold underline">{"Hello world!"}</h1>
                    </div>

                    <div class="p-3">
                        {"body content"}
                    </div>
                </div>
            </div>
        </div>
    }
}

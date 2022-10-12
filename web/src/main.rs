use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

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

fn main() {
    yew::Renderer::<App>::new().render();
}

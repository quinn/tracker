mod todo_list;
pub use self::todo_list::TodoList;

use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <div class="bg-neutral-900 text-lime-200">
            <div class="root">
                <div class="border-r border-neutral-700 p-3">
                    {"Sidebar"}
                </div>

                <div>
                    <div class="border-b border-neutral-700 p-3">
                        <h1 class="text-3xwl font-bold underline">{"Hello world!"}</h1>
                    </div>

                    <div class="p-3">
                        <Suspense {fallback}>
                            <TodoList />
                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    }
}

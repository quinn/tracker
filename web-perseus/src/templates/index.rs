use gloo_console::log;
use gloo_timers::callback::Interval;
use perseus::state::Freeze;
use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
    pub loading: bool,
}

#[perseus::build_state]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> perseus::RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: "Hello World!".to_string(),
        loading: true,
    })
}

#[perseus::template_rx]
pub fn index_page<'a, G: Html>(cx: Scope<'a>, state: IndexPageStateRx<'a>) -> View<G> {
    let set_greeting = |_| state.greeting.set("change greeting".into());
    let render_ctx = perseus::get_render_ctx!(cx);

    view! { cx,
        // Don't worry, there are much better ways of styling in Perseus!
        div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh;") {
            h1(class="ui header") { "Welcome to Perseus!" }
            p {
                "This is just an example app. Try changing some code inside "
                code { (state.greeting.get()) }
                button(on:click=set_greeting) { "click me" }
                " and you'll be able to see the results here!"
            }

            button(id = "freeze_button", on:click = |_| {
                log!(render_ctx.freeze());
            }) { "Freeze!" }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome to Perseus!" }
        link(
            rel="stylesheet",
            href="https://cdnjs.cloudflare.com/ajax/libs/semantic-ui/2.5.0/semantic.min.css",
            integrity="sha512-KXol4x3sVoO+8ZsWPFI/r5KBVB/ssCGB5tsv2nVOKwLg33wTFP3fmnXa47FdSVIshVTgsYk/1734xSk9aFIa4A==",
            crossorigin="anonymous",
            referrerpolicy="no-referrer")
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

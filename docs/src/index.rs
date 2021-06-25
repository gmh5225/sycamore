use sycamore::prelude::*;
use sycamore_router::Link;

#[component(Index<G>)]
pub fn index() -> Template<G> {
    template! {
        div(class="flex flex-col items-center w-full") {
            h1(class="text-5xl font-bold mt-20 mb-5") {
                "Sycamore"
            }

            p(class="mb-10") {
                "Pure Rust + WASM web-apps"
            }

            Link(("/getting_started/installation", template! {
                span(class="py-2 px-3 bg-white hover:bg-yellow-500 border-2 border-yellow-500 \
                    rounded font-medium transition") {
                    "Read the Book"
                }
            }))
        }
    }
}
mod components;
mod utils;
mod pages;

use components::footer::Footer;
use components::navbar::Navbar;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use pages::blog::Blog;
use pages::home::Home;
use pages::post::PostPage;
use utils::content::fetch_all_posts;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    let posts_resource = LocalResource::new(|| async move {
        fetch_all_posts().await
    });
    provide_context(posts_resource);

    view! {
        <Router>
            <div style="display: flex; flex-direction: column; min-height: 100vh;">
                <Navbar/>
                <main style="flex: 1; display: flex; flex-direction: column;">
                    <Routes fallback=|| view! { <h2>"404 Not Found"</h2> }>
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/blog") view=Blog/>
                        <Route path=path!("/blog/:slug") view=PostPage/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    if let Some(loader) = document.get_element_by_id("loading-layer") {
        loader.remove();
    }

    leptos::mount::mount_to_body(App);
}

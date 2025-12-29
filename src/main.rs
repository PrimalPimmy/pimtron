mod components;
mod pages;
mod utils;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::pages::blog_list::BlogList;
use crate::pages::home::Home;
use crate::pages::post::PostPage;

stylance::import_style!(_vars, "styles/variables.module.css");
stylance::import_style!(_app, "styles/app.module.css");

fn main() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    if let Some(loader) = document.get_element_by_id("loading-layer") {
        loader.remove();
    }

    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    // Eagerly fetch posts as soon as the app starts
    let posts_resource = LocalResource::new(|| async {
        crate::utils::content::fetch_all_posts().await
    });
    provide_context(posts_resource);

    view! {
        <MetaTags />
        <Stylesheet id="leptos" href="/pkg/pimtron.css" />
        <Title text="Pimtron" />
        <Router>
            <div class="main-layout" style="display: flex; flex-direction: column; min-height: 100vh;">
                <Navbar />
                <main class="content" style="flex: 1; display: flex; flex-direction: column;">
                    <Routes fallback=|| view! { <h2>"404 Not Found"</h2> }>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/blog") view=BlogList />
                        <Route path=path!("/blog/:slug") view=PostPage />
                    </Routes>
                </main>
                <Footer />
            </div>
        </Router>
    }
}

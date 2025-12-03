mod components;
mod pages;
mod content;

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;
use pages::home::Home;
use pages::blog::Blog;
use pages::post::PostPage;
use components::navbar::Navbar;
use components::footer::Footer;

#[component]
fn App() -> impl IntoView {
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
    leptos::mount::mount_to_body(App);
}
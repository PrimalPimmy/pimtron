use leptos::prelude::*;
stylance::import_style!(style, "../styles/navbar.module.css");

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class=style::nav>
            <a href="/" class=style::logo>"Pimtron"</a>
            <div class=style::links>
                <a href="/" class=style::link>"Home"</a>
                <a href="/blog" class=style::link>"Blog"</a>
            </div>
        </nav>
    }
}
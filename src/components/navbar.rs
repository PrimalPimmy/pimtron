use leptos::prelude::*;
stylance::import_style!(
    #[allow(unused)]
    style,
    "../styles/navbar_footer.module.css"
);

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class=style::nav>
            <div class=style::links>
                <a href="/" class=style::link>"Home"</a>
                <a href="/about" class=style::link>"About"</a>
                <a href="/projects" class=style::link>"Projects"</a>
                <a href="/blog" class=style::link>"Blog"</a>
            </div>
        </nav>
    }
}

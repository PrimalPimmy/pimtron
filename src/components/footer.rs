use leptos::prelude::*;
stylance::import_style!(style, "../styles/footer.module.css");

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class=style::footer>
            <p class=style::text>"© 2025 Pimtron. All systems normal."</p>
        </footer>
    }
}

use leptos::prelude::*;
stylance::import_style!(
    #[allow(unused)]
    style,
    "../styles/navbar_footer.module.css"
);

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class=style::footer>
            <p class=style::text>"© 2025 Pimtron. Made with " <span class=style::footer_highlight> "Leptos. " </span> "Deployed with " <span class=style::footer_highlight> "Nix. " </span> "All systems normal."</p>
        </footer>
    }
}

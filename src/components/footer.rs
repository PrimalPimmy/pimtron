use leptos::prelude::*;

stylance::import_style!(
    #[allow(unused)]
    style,
    "../styles/navbar_footer.module.css"
);

#[component]
pub fn Footer() -> impl IntoView {
    // read initial theme synchronously
    let initial_dark = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("theme").ok().flatten())
        .is_some_and(|t| t == "dark");

    let (is_dark, set_is_dark) = signal(initial_dark);

    // single effect: apply theme to DOM and persist to localStorage
    Effect::new(move |_| {
        let dark = is_dark.get();
        if let Some(window) = web_sys::window() {
            let theme = if dark { "dark" } else { "light" };
            if let Some(el) = window.document().and_then(|d| d.document_element()) {
                let _ = el.set_attribute("data-theme", theme);
            }
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", theme);
            }
        }
    });

    let toggle_theme = move |_| {
        set_is_dark.update(|dark| *dark = !*dark);
    };

    view! {
        <footer class=style::footer>
            <p class=style::text>"© 2026 Pimtron. Made with " <span class=style::footer_highlight> "Leptos. " </span> "Deployed with " <span class=style::footer_highlight> "Nix. " </span> "All systems normal."</p>
            <button class=style::theme_switcher on:click=toggle_theme>
                {move || if is_dark.get() { "[ ☾ DARK ]" } else { "[ ☀ LIGHT ]" }}
            </button>
        </footer>
    }
}

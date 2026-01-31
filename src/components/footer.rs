use leptos::prelude::*;

stylance::import_style!(
    #[allow(unused)]
    style,
    "../styles/navbar_footer.module.css"
);

#[component]
pub fn Footer() -> impl IntoView {
    // Use a simple leptos signal for reactivity
    let (is_dark, set_is_dark) = signal(false);

    // Load theme from localStorage on mount
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved_theme)) = storage.get_item("theme") {
                    let dark = saved_theme == "dark";
                    set_is_dark.set(dark);
                    // Also apply immediately
                    if let Some(element) = window.document().and_then(|d| d.document_element()) {
                        let _ = element
                            .set_attribute("data-theme", if dark { "dark" } else { "light" });
                    }
                }
            }
        }
    });

    // Apply theme whenever is_dark changes
    Effect::new(move |_| {
        let dark = is_dark.get();
        if let Some(window) = web_sys::window() {
            // Update DOM
            if let Some(element) = window.document().and_then(|d| d.document_element()) {
                let _ = element.set_attribute("data-theme", if dark { "dark" } else { "light" });
            }
            // Save to localStorage
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("theme", if dark { "dark" } else { "light" });
            }
        }
    });

    let toggle_theme = move |_| {
        set_is_dark.set(!is_dark.get());
    };

    view! {
        <footer class=style::footer>
            <p class=style::text>"© 2025 Pimtron. Made with " <span class=style::footer_highlight> "Leptos. " </span> "Deployed with " <span class=style::footer_highlight> "Nix. " </span> "All systems normal."</p>
            <button class=style::theme_switcher on:click=toggle_theme>
                {move || if is_dark.get() { "[ ☾ DARK ]" } else { "[ ☀ LIGHT ]" }}
            </button>
        </footer>
    }
}

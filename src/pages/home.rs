use leptos::prelude::*;
stylance::import_style!(style, "../styles/home.module.css");

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class=style::home_container>
            <div class=style::content_wrapper>
                <h1 class=style::hero_title>
                    "Hey, I'm Prashant a.k.a Pimtron/Pimmy"
                </h1>
                <p class=style::hero_subtitle>
                    "I'm a " <span class=style::highlight>"Software Developer"</span>
                    ". I love to talk about the wide spectrum in Tech."

                </p>
                <div class=style::actions>
                    <a href="/blog" class=style::btn_primary>"Read my blog"</a>
                    <a href="https://github.com/primalpimmy" target="_blank" class=style::btn_secondary>"Check GitHub"</a>
                </div>
            </div>
        </div>
    }
}

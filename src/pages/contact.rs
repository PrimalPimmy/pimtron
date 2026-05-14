use leptos::prelude::*;
use leptos_meta::*;

stylance::import_style!(
    #[allow(unused)]
    common,
    "../styles/common.module.css"
);
stylance::import_style!(style, "../styles/contact.module.css");

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Title text="Pimtron - Contact" />
        <div class=common::container>
            <h1 class=common::page_title>"CONTACT"</h1>

            <div class=style::contact_grid>
                // EMAIL
                <a href="mailto:prashant20.pm@gmail.com" class=style::contact_card>
                    <div class=style::card_icon>"✉"</div>
                    <div class=style::card_content>
                        <span class=style::card_label>"EMAIL"</span>
                        <span class=style::card_value>"prashant20.pm@gmail.com"</span>
                    </div>
                    <span class=style::card_arrow>"↗"</span>
                </a>

                // DISCORD
                <div class=style::contact_card>
                    <div class=style::card_icon>"⊕"</div>
                    <div class=style::card_content>
                        <span class=style::card_label>"DISCORD"</span>
                        <span class=style::card_value>"@pimtronous"</span>
                    </div>
                </div>
            </div>

            <div class=common::back_link_container>
                <a href="/" class=common::back_link>"< Back to Home"</a>
            </div>
        </div>
    }
}

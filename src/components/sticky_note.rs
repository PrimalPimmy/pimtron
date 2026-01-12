use leptos::ev;
use leptos::html::Div;
use leptos::leptos_dom::helpers::window_event_listener;
use leptos::prelude::*;
use web_sys::wasm_bindgen::JsCast;
stylance::import_style!(style, "../styles/sticky_note.module.css");

#[component]
pub fn StickyNote(
    #[prop(default = 0.0)] initial_x: f64,
    #[prop(default = 0.0)] initial_y: f64,
    children: Children,
) -> impl IntoView {
    // Current position state
    let (pos, set_pos) = signal((initial_x, initial_y));

    // Drag Context: Stores (mouse_offset_x, mouse_offset_y)
    // We don't need container offset here as we are positioning absolutely relative to the offset parent
    let (drag_ctx, set_drag_ctx) = signal(Option::<(f64, f64)>::None);

    let note_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        // Global mouse listeners for this specific note
        let move_listener = window_event_listener(ev::mousemove, move |ev| {
            if let Some((off_x, off_y)) = drag_ctx.get() {
                // Update position based on mouse - offset
                // We rely on the parent being `position: relative`
                // However, ev.client_x is global.
                // To get correct position relative to parent, we need to account for parent's position?
                // Actually, if we just use delta it might be easier, but let's stick to absolute calculation.
                // We need the parent's bounding rect to calculate 'left' correctly if we use client_x.
                // ALTERNATIVE: Just stick to the logic we had:
                // new_x = ev.client_x - parent_left - offset_x

                // To do this inside the component without passing parent ref, we can calculate parent rect from the element itself?
                if let Some(note) = note_ref.get()
                    && let Some(parent) = note.offset_parent() {
                        let parent_rect = parent.get_bounding_client_rect();
                        let new_x = ev.client_x() as f64 - parent_rect.left() - off_x;
                        let new_y = ev.client_y() as f64 - parent_rect.top() - off_y;
                        set_pos.set((new_x, new_y));
                    }
            }
        });

        let up_listener = window_event_listener(ev::mouseup, move |_| {
            set_drag_ctx.set(None);
        });

        on_cleanup(move || {
            move_listener.remove();
            up_listener.remove();
        });
    });

    view! {
        <div
            node_ref=note_ref
            class=style::sticky_note
            style:left=move || format!("{}px", pos.get().0)
            style:top=move || format!("{}px", pos.get().1)
            style:cursor=move || if drag_ctx.get().is_some() { "grabbing" } else { "grab" }
            on:mousedown=move |ev| {
                // Determine offset of mouse relative to the note's top-left
                let rect = ev.target().unwrap().unchecked_into::<web_sys::Element>().get_bounding_client_rect();
                let off_x = ev.client_x() as f64 - rect.left();
                let off_y = ev.client_y() as f64 - rect.top();
                set_drag_ctx.set(Some((off_x, off_y)));
            }
        >
            {children()}
        </div>
    }
}

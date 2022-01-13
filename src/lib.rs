use js_sys::Date;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{KeyboardEvent, HtmlElement, Document};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    setup_keypress(&document);
    setup_keydown(&document);

    Ok(())
}

fn setup_keypress(document: &Document) {
    let input = document.get_element_by_id("input-display").unwrap();
    let command_output = document.get_element_by_id("command_output").unwrap();

    let callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        event.prevent_default();

        let input = input.dyn_ref::<HtmlElement>().unwrap();
        let command_output = command_output.dyn_ref::<HtmlElement>().unwrap();

        let mut text = input.inner_text();

        match &*event.key() {
            "Enter" => if !text.is_empty() {
                text = "".to_string();
                command_output.set_inner_text(&format!("[{}] unimplemented", Date::now().to_string()));
            },
            "Backspace" => if !text.is_empty() {
                text = text[..text.len() - 1].to_string();
            },
            key => text = format!("{}{}", input.inner_text(), key),
        }

        input.set_inner_text(&text);
    }) as Box<dyn FnMut(_)>);

    document.set_onkeypress(Some(callback.as_ref().unchecked_ref()));
    callback.forget();
}

fn setup_keydown(document: &Document) {
    let input = document.get_element_by_id("input-display").unwrap();

    let callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let input = input.dyn_ref::<HtmlElement>().unwrap();

        if event.key() == "Backspace" {
            let mut text = input.inner_text();

            if !text.is_empty() {
                let mut chars = text.chars();
                chars.next_back();
                text = chars.collect();
            }

            input.set_inner_text(&text);
        }
    }) as Box<dyn FnMut(_)>);

    document.set_onkeydown(Some(callback.as_ref().unchecked_ref()));
    callback.forget();
}

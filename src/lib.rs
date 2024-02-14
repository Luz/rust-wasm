use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    let body = document.body().expect("no body");
    let val = document.create_element("p")?;
    val.set_text_content(Some("main function called"));

    let _ = body.append_child(&val)?;

    let _ = addcontent("from rust".to_owned());

    Ok(())
}

#[wasm_bindgen]
pub fn addcontent(source: String) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document");
    let body = document.body().expect("no body");
    let val = document.create_element("p")?;
    val.set_text_content(Some(&("addcontent() called ".to_owned() + &source)));

    let _ = body.append_child(&val)?;

    Ok(())
}


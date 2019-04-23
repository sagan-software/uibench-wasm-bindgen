use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type TableItemState;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &TableItemState) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn active(this: &TableItemState) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn props(this: &TableItemState) -> Box<[JsValue]>;
}

#[wasm_bindgen]
extern "C" {
    pub type TableState;

    #[wasm_bindgen(method, getter)]
    pub fn items(this: &TableState) -> Box<[JsValue]>;
}

#[wasm_bindgen]
extern "C" {
    pub type AnimBoxState;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &AnimBoxState) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn time(this: &AnimBoxState) -> f32;
}

#[wasm_bindgen]
extern "C" {
    pub type AnimState;

    #[wasm_bindgen(method, getter)]
    pub fn items(this: &AnimState) -> Box<[JsValue]>;
}

#[wasm_bindgen]
extern "C" {
    pub type TreeNodeState;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &TreeNodeState) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn container(this: &TreeNodeState) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn children(this: &TreeNodeState) -> Option<Box<[JsValue]>>;
}

#[wasm_bindgen]
extern "C" {
    pub type TreeState;

    #[wasm_bindgen(method, getter)]
    pub fn root(this: &TreeState) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    pub type AppState;

    #[wasm_bindgen(method, getter)]
    pub fn location(this: &AppState) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn table(this: &AppState) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn anim(this: &AppState) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn tree(this: &AppState) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = uibench)]
    pub fn init(name: &str, version: &str);

    #[wasm_bindgen(js_namespace = uibench)]
    pub fn run(update: &JsValue, finish: &JsValue);
}

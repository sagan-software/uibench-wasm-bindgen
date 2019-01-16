use serde_derive::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Event, HtmlElement};

#[derive(Deserialize)]
struct TableItemState {
    id: u32,
    active: bool,
    props: Vec<String>,
}

#[derive(Deserialize)]
struct TableState {
    items: Vec<TableItemState>,
}

#[derive(Deserialize)]
struct AnimBoxState {
    id: u32,
    time: f32,
}

#[derive(Deserialize)]
struct AnimState {
    items: Vec<AnimBoxState>,
}

#[derive(Deserialize)]
struct TreeNodeState {
    id: u32,
    container: bool,
    children: Option<Vec<TreeNodeState>>,
}

#[derive(Deserialize)]
struct TreeState {
    root: TreeNodeState,
}

#[derive(Deserialize)]
struct AppState {
    location: String,
    table: TableState,
    anim: AnimState,
    tree: TreeState,
}

mod uibench {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = uibench)]
        pub fn init(name: &str, version: &str);

        #[wasm_bindgen(js_namespace = uibench)]
        pub fn run(update: &JsValue, finish: &JsValue);
    }
}

fn render_table_cell(html: &mut String, props: &str) {
    html.push_str("<td class='TableCell' data-text='");
    html.push_str(props);
    html.push_str("'>");
    html.push_str(props);
    html.push_str("</td>");
}

fn render_table_row(html: &mut String, data: &TableItemState) {
    html.push_str("<tr class='TableRow");
    if data.active {
        html.push_str(" active");
    }
    html.push_str("' data-id='");

    let id = data.id.to_string();
    html.push_str(&id);
    html.push_str("'>");

    let mut pound_id = String::from("#");
    pound_id.push_str(&id);
    render_table_cell(html, &pound_id);

    for prop in data.props.iter() {
        render_table_cell(html, prop);
    }
    html.push_str("</tr>");
}

fn render_table(html: &mut String, data: &TableState) {
    html.push_str("<table class='Table'><tbody>");
    for item in data.items.iter() {
        render_table_row(html, item);
    }
    html.push_str("</tbody></table>");
}

fn render_anim_box(html: &mut String, props: &AnimBoxState) {
    html.push_str("<div class='AnimBox' style='border-radius:");
    let border_radius = props.time % 10.0;
    html.push_str(&border_radius.to_string());
    html.push_str("px;background:rgba(0,0,0,");
    let alpha = border_radius / 10.0 + 0.5;
    html.push_str(&alpha.to_string());
    html.push_str(")' data-id='");
    html.push_str(&props.id.to_string());
    html.push_str("'></div>");
}

fn render_anim(html: &mut String, props: &AnimState) {
    html.push_str("<div class='Anim'>");
    for item in props.items.iter() {
        render_anim_box(html, item);
    }
    html.push_str("</div>")
}

fn render_tree_leaf(html: &mut String, props: &TreeNodeState) {
    html.push_str("<li class='TreeLeaf'>");
    html.push_str(&props.id.to_string());
    html.push_str("</li>");
}

fn render_tree_node(html: &mut String, props: &TreeNodeState) {
    html.push_str("<ul class='TreeNode'>");
    if let Some(children) = &props.children {
        for child in children {
            if child.container {
                render_tree_node(html, child);
            } else {
                render_tree_leaf(html, child);
            }
        }
    }
    html.push_str("</ul>");
}

fn render_tree(html: &mut String, props: &TreeState) {
    html.push_str("<div class='Tree'>");
    render_tree_node(html, &props.root);
    html.push_str("</div>");
}

fn render_main(html: &mut String, data: &AppState) {
    html.push_str("<div class='Main'>");
    match data.location.as_str() {
        "table" => render_table(html, &data.table),
        "anim" => render_anim(html, &data.anim),
        "tree" => render_tree(html, &data.tree),
        _ => (),
    }
    html.push_str("</div>");
}

fn handle_click(e: Event) {
    let target = e.target().unwrap();
    let el = target.unchecked_ref::<HtmlElement>();
    let class_name = el.class_name();
    if class_name == "TableCell" {
        let text = el.get_attribute("data-text").unwrap();
        console::log_2(&JsValue::from_str("Click"), &JsValue::from_str(&text));
        e.prevent_default();
        e.stop_propagation();
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let container = document.get_element_by_id("App").unwrap();

    let mut inner_html = String::with_capacity(75_000);
    let update = Closure::wrap(Box::new(move |value: JsValue| {
        inner_html.clear();
        let state: AppState = value.into_serde().unwrap();
        render_main(&mut inner_html, &state);
        container.set_inner_html(&inner_html);

        if state.location == "table" {
            let onclick = Closure::wrap(Box::new(handle_click) as Box<dyn FnMut(_)>);

            let cells = document.query_selector_all(".TableCell").unwrap();
            for i in 0..cells.length() {
                cells
                    .get(i)
                    .unwrap()
                    .unchecked_ref::<HtmlElement>()
                    .set_onclick(Some(onclick.as_ref().unchecked_ref()));
            }
            onclick.forget();
        }
    }) as Box<dyn FnMut(_)>);

    let document = window.document().unwrap();
    let finish = Closure::wrap(Box::new(move |value: JsValue| {
        let mut inner_html = String::new();
        inner_html.push_str("<pre>");
        let stringified: String = js_sys::JSON::stringify(&value).unwrap().into();
        inner_html.push_str(&stringified);
        inner_html.push_str("</pre>");
        let body = document.body().unwrap();
        body.set_inner_html(&inner_html);
    }) as Box<dyn FnMut(_)>);

    uibench::init("wasm-bindgen", "0.2.31");
    uibench::run(
        update.as_ref().unchecked_ref(),
        finish.as_ref().unchecked_ref(),
    );

    update.forget();
    finish.forget();
}

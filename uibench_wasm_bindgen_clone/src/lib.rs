use std::borrow::Cow;
use std::collections::HashMap;
use uibench_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Document, Element, Event, HtmlElement, HtmlTemplateElement, Node};

type CowStr = Cow<'static, str>;

struct Context {
    document: Document,
    compiler: HtmlTemplateElement,
    templates: HashMap<CowStr, Element>,
}

impl Context {
    fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let compiler = document
            .create_element("template")
            .unwrap()
            .unchecked_into::<web_sys::HtmlTemplateElement>();
        let templates = HashMap::new();
        Context {
            document,
            compiler,
            templates,
        }
    }

    pub fn get_template<S: Into<CowStr>>(&mut self, s: S) -> HtmlElement {
        let html = s.into();
        match self.templates.get(&html) {
            Some(template) => template
                .clone_node_with_deep(true)
                .unwrap()
                .unchecked_into::<HtmlElement>(),
            None => {
                self.compiler.set_inner_html(&html);
                let template = self.compiler.content().first_element_child().unwrap();
                let el = template.clone_node_with_deep(true).unwrap();
                self.templates.insert(html, template);
                el.unchecked_into::<HtmlElement>()
            }
        }
    }
}

const TABLE_CELL: &str = "<td class=TableCell></td>";

fn render_table_cell(ctx: &mut Context, props: &str) -> HtmlElement {
    let el = ctx.get_template(TABLE_CELL);
    el.set_attribute("data-text", props);
    el.set_text_content(Some(props));
    el
}

const TABLE_ROW: &str = "<tr class=TableRow></tr>";

fn render_table_row(ctx: &mut Context, data: &TableItemState) -> HtmlElement {
    let el = ctx.get_template(TABLE_ROW);
    if data.active() {
        el.class_list().add_1("active");
    }
    let id = data.id().to_string();
    el.set_attribute("data-id", &id);

    let mut pound_id = String::from("#");
    pound_id.push_str(&id);

    el.append_child(&render_table_cell(ctx, &pound_id)).unwrap();

    for prop in data.props().iter() {
        let value: String = prop.unchecked_ref::<js_sys::JsString>().into();
        el.append_child(&render_table_cell(ctx, &value)).unwrap();
    }
    el
}

const TABLE: &str = "<table class=Table><tbody></tbody></table>";

fn render_table(ctx: &mut Context, data: &TableState) -> HtmlElement {
    let el = ctx.get_template(TABLE);
    let tbody = el.first_element_child().unwrap();
    for item in data.items().iter() {
        tbody
            .append_child(&render_table_row(
                ctx,
                item.unchecked_ref::<TableItemState>(),
            ))
            .unwrap();
    }
    el
}

const ANIM_BOX: &str = "<div class=AnimBox></div>";

fn render_anim_box(ctx: &mut Context, props: &AnimBoxState) -> HtmlElement {
    let el = ctx.get_template(ANIM_BOX);
    let style = el.style();

    let border_radius_value = props.time() % 10.0;
    let mut border_radius_string = border_radius_value.to_string();
    border_radius_string.push_str("px");
    style.set_property("border-radius", &border_radius_string);

    let alpha_value = border_radius_value / 10.0 + 0.5;
    let mut alpha_string = "rgba(0,0,0,".to_string();
    alpha_string.push_str(&alpha_value.to_string());
    style.set_property("background", &alpha_string);

    el.set_attribute("data-id", &props.id().to_string());
    el
}

const ANIM: &str = "<div class=Anim></div>";

fn render_anim(ctx: &mut Context, props: &AnimState) -> HtmlElement {
    let el = ctx.get_template(ANIM);
    for item in props.items().iter() {
        el.append_child(&render_anim_box(ctx, item.unchecked_ref::<AnimBoxState>()))
            .unwrap();
    }
    el
}

const TREE_LEAF: &str = "<li class=TreeLeaf></li>";

fn render_tree_leaf(ctx: &mut Context, props: &TreeNodeState) -> HtmlElement {
    let el = ctx.get_template(TREE_LEAF);
    el.set_text_content(Some(&props.id().to_string()));
    el
}

const TREE_NODE: &str = "<ul class=TreeNode></ul>";

fn render_tree_node(ctx: &mut Context, props: &TreeNodeState) -> HtmlElement {
    let el = ctx.get_template(TREE_NODE);
    if let Some(children) = &props.children() {
        for value in children.iter() {
            let child = value.unchecked_ref::<TreeNodeState>();
            if child.container() {
                el.append_child(&render_tree_node(ctx, child)).unwrap();
            } else {
                el.append_child(&render_tree_leaf(ctx, child)).unwrap();
            }
        }
    }
    el
}

const TREE: &str = "<div class=Tree></div>";

fn render_tree(ctx: &mut Context, props: &TreeState) -> HtmlElement {
    let el = ctx.get_template(TREE);
    el.append_child(&render_tree_node(
        ctx,
        &props.root().unchecked_ref::<TreeNodeState>(),
    ))
    .unwrap();
    el
}

const MAIN: &str = "<div class=Main></div>";

fn render_main(ctx: &mut Context, data: &AppState) -> HtmlElement {
    let el = ctx.get_template(MAIN);
    match data.location().as_str() {
        "table" => {
            el.append_child(&render_table(
                ctx,
                &data.table().unchecked_ref::<TableState>(),
            ))
            .unwrap();
        }
        "anim" => {
            el.append_child(&render_anim(ctx, &data.anim().unchecked_ref::<AnimState>()))
                .unwrap();
        }
        "tree" => {
            el.append_child(&render_tree(ctx, &data.tree().unchecked_ref::<TreeState>()))
                .unwrap();
        }
        _ => (),
    }
    el
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
    let onclick = Closure::wrap(Box::new(handle_click) as Box<dyn FnMut(_)>);
    container
        .unchecked_ref::<HtmlElement>()
        .set_onclick(Some(onclick.as_ref().unchecked_ref()));

    let mut context = Context::new();

    let update = Closure::wrap(Box::new(move |value: JsValue| {
        let state = value.unchecked_ref::<AppState>();
        let el = render_main(&mut context, &state);
        container.set_text_content(None);
        container.append_child(&el).unwrap();
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

    uibench_sys::init("wasm-bindgen (cloneNode)", "0.2.31");
    uibench_sys::run(
        update.as_ref().unchecked_ref(),
        finish.as_ref().unchecked_ref(),
    );

    update.forget();
    finish.forget();
}

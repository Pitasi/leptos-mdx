use std::collections::HashMap;

use crate::markdown::parse;
use html_parser::{Dom, Element};
use leptos::{
    component, html::ElementDescriptor, warn, Children, Fragment, HtmlElement, IntoView, View, Scope,
};

#[component]
/// Renders a markdown source into a Leptos component.
/// Custom components can be used in the markdown source.
pub fn Mdx(cx: Scope, source: String, components: Components) -> impl IntoView {
    let (_fm, html) = parse(&source).expect("invalid mdx");
    // TODO: we could expose frontmatter in the context so components can use its value

    let dom = Dom::parse(&html).expect("invalid html");

    let mut root_views = vec![];
    for node in dom.children {
        if let Some(el) = node.element() {
            root_views.push(process_element(cx, el, &components));
        }
    }

    Fragment::new(root_views)
}

/// Props passed to a custom component.
pub struct MdxComponentProps {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, Option<String>>,
    pub children: Children,
}

/// A collection of custom components.
pub struct Components {
    cx: Scope,
    components: HashMap<String, Box<dyn Fn(MdxComponentProps) -> View>>,
}

impl Components {
    pub fn new(cx: Scope) -> Self {
        Self {
            cx: cx,
            components: HashMap::new(),
        }
    }

    /// Register a new custom component that won't receive any props.
    pub fn add<F, IV>(&mut self, name: String, component: F)
    where
        F: Fn(Scope) -> IV + 'static,
        IV: IntoView + 'static,
    {
        let context = self.cx;
        self.components
            .insert(name, Box::new(move |_| component(context).into_view(context)));
    }

    /// Register a new custom component that will receive props. The standardized
    /// MdxComponentsProps are converted to the props type of the component using the provided
    /// adapter.
    pub fn add_props<F, IV, Props, PropsFn>(
        &mut self,
        name: String,
        component: F,
        props_adapter: PropsFn,
    ) where
        F: Fn(Scope, Props) -> IV + 'static,
        IV: IntoView + 'static,
        PropsFn: Fn(MdxComponentProps) -> Props + 'static,
    {
        let context = self.cx;
        self.components.insert(
            name,
            Box::new(move |props| component(context,props_adapter(props)).into_view(context)),
        );
    }

    fn get(&self, name: &str) -> Option<&Box<dyn Fn(MdxComponentProps) -> View>> {
        self.components.get(name)
    }
}

pub fn process_element(cx: Scope, el: &Element, components: &Components) -> View {
    let mut child_views = vec![];
    for child in &el.children {
        match child {
            html_parser::Node::Element(el_child) => {
                child_views.push(process_element(cx, el_child, components));
            }
            html_parser::Node::Text(text) => {
                child_views.push(text.clone().into_view(cx));
            }
            _ => {}
        }
    }

    // Custom elements
    if let Some(component) = components.get(&el.name) {
        let cmp = component(MdxComponentProps {
            id: el.id.clone(),
            classes: el.classes.clone(),
            attributes: el.attributes.clone(),
            children: Box::new(move |_| Fragment::new(child_views)),
        });
        return cmp;
    }

    // HTML elements
    match el.name.as_str() {
        "html" => html_element(cx, el, child_views, leptos::html::html(cx)),
        "base" => html_element(cx, el, child_views, leptos::html::base(cx)),
        "head" => html_element(cx, el, child_views, leptos::html::head(cx)),
        "link" => html_element(cx, el, child_views, leptos::html::link(cx)),
        "meta" => html_element(cx, el, child_views, leptos::html::meta(cx)),
        "style" => html_element(cx, el, child_views, leptos::html::style(cx)),
        "title" => html_element(cx, el, child_views, leptos::html::title(cx)),
        "body" => html_element(cx, el, child_views, leptos::html::body(cx)),
        "address" => html_element(cx, el, child_views, leptos::html::address(cx)),
        "article" => html_element(cx, el, child_views, leptos::html::article(cx)),
        "aside" => html_element(cx, el, child_views, leptos::html::aside(cx)),
        "footer" => html_element(cx, el, child_views, leptos::html::footer(cx)),
        "header" => html_element(cx, el, child_views, leptos::html::header(cx)),
        "hgroup" => html_element(cx, el, child_views, leptos::html::hgroup(cx)),
        "h1" => html_element(cx, el, child_views, leptos::html::h1(cx)),
        "h2" => html_element(cx, el, child_views, leptos::html::h2(cx)),
        "h3" => html_element(cx, el, child_views, leptos::html::h3(cx)),
        "h4" => html_element(cx, el, child_views, leptos::html::h4(cx)),
        "h5" => html_element(cx, el, child_views, leptos::html::h5(cx)),
        "h6" => html_element(cx, el, child_views, leptos::html::h6(cx)),
        "main" => html_element(cx, el, child_views, leptos::html::main(cx)),
        "nav" => html_element(cx, el, child_views, leptos::html::nav(cx)),
        "section" => html_element(cx, el, child_views, leptos::html::section(cx)),
        "blockquote" => html_element(cx, el, child_views, leptos::html::blockquote(cx)),
        "dd" => html_element(cx, el, child_views, leptos::html::dd(cx)),
        "div" => html_element(cx, el, child_views, leptos::html::div(cx)),
        "dl" => html_element(cx, el, child_views, leptos::html::dl(cx)),
        "dt" => html_element(cx, el, child_views, leptos::html::dt(cx)),
        "figcaption" => html_element(cx, el, child_views, leptos::html::figcaption(cx)),
        "figure" => html_element(cx, el, child_views, leptos::html::figure(cx)),
        "hr" => html_element(cx, el, child_views, leptos::html::hr(cx)),
        "li" => html_element(cx, el, child_views, leptos::html::li(cx)),
        "ol" => html_element(cx, el, child_views, leptos::html::ol(cx)),
        "p" => html_element(cx, el, child_views, leptos::html::p(cx)),
        "pre" => html_element(cx, el, child_views, leptos::html::pre(cx)),
        "ul" => html_element(cx, el, child_views, leptos::html::ul(cx)),
        "a" => html_element(cx, el, child_views, leptos::html::a(cx)),
        "abbr" => html_element(cx, el, child_views, leptos::html::abbr(cx)),
        "b" => html_element(cx, el, child_views, leptos::html::b(cx)),
        "bdi" => html_element(cx, el, child_views, leptos::html::bdi(cx)),
        "bdo" => html_element(cx, el, child_views, leptos::html::bdo(cx)),
        "br" => html_element(cx, el, child_views, leptos::html::br(cx)),
        "cite" => html_element(cx, el, child_views, leptos::html::cite(cx)),
        "code" => html_element(cx, el, child_views, leptos::html::code(cx)),
        "data" => html_element(cx, el, child_views, leptos::html::data(cx)),
        "dfn" => html_element(cx, el, child_views, leptos::html::dfn(cx)),
        "em" => html_element(cx, el, child_views, leptos::html::em(cx)),
        "i" => html_element(cx, el, child_views, leptos::html::i(cx)),
        "kbd" => html_element(cx, el, child_views, leptos::html::kbd(cx)),
        "mark" => html_element(cx, el, child_views, leptos::html::mark(cx)),
        "q" => html_element(cx, el, child_views, leptos::html::q(cx)),
        "rp" => html_element(cx, el, child_views, leptos::html::rp(cx)),
        "rt" => html_element(cx, el, child_views, leptos::html::rt(cx)),
        "ruby" => html_element(cx, el, child_views, leptos::html::ruby(cx)),
        "s" => html_element(cx, el, child_views, leptos::html::s(cx)),
        "samp" => html_element(cx, el, child_views, leptos::html::samp(cx)),
        "small" => html_element(cx, el, child_views, leptos::html::small(cx)),
        "span" => html_element(cx, el, child_views, leptos::html::span(cx)),
        "strong" => html_element(cx, el, child_views, leptos::html::strong(cx)),
        "sub" => html_element(cx, el, child_views, leptos::html::sub(cx)),
        "sup" => html_element(cx, el, child_views, leptos::html::sup(cx)),
        "time" => html_element(cx, el, child_views, leptos::html::time(cx)),
        "u" => html_element(cx, el, child_views, leptos::html::u(cx)),
        "var" => html_element(cx, el, child_views, leptos::html::var(cx)),
        "wbr" => html_element(cx, el, child_views, leptos::html::wbr(cx)),
        "area" => html_element(cx, el, child_views, leptos::html::area(cx)),
        "audio" => html_element(cx, el, child_views, leptos::html::audio(cx)),
        "img" => html_element(cx, el, child_views, leptos::html::img(cx)),
        "map" => html_element(cx, el, child_views, leptos::html::map(cx)),
        "track" => html_element(cx, el, child_views, leptos::html::track(cx)),
        "video" => html_element(cx, el, child_views, leptos::html::video(cx)),
        "embed" => html_element(cx, el, child_views, leptos::html::embed(cx)),
        "iframe" => html_element(cx, el, child_views, leptos::html::iframe(cx)),
        "object" => html_element(cx, el, child_views, leptos::html::object(cx)),
        "param" => html_element(cx, el, child_views, leptos::html::param(cx)),
        "picture" => html_element(cx, el, child_views, leptos::html::picture(cx)),
        "portal" => html_element(cx, el, child_views, leptos::html::portal(cx)),
        "source" => html_element(cx, el, child_views, leptos::html::source(cx)),
        "svg" => html_element(cx, el, child_views, leptos::html::svg(cx)),
        "math" => html_element(cx, el, child_views, leptos::html::math(cx)),
        "canvas" => html_element(cx, el, child_views, leptos::html::canvas(cx)),
        "noscript" => html_element(cx, el, child_views, leptos::html::noscript(cx)),
        "script" => html_element(cx, el, child_views, leptos::html::script(cx)),
        "del" => html_element(cx, el, child_views, leptos::html::del(cx)),
        "ins" => html_element(cx, el, child_views, leptos::html::ins(cx)),
        "caption" => html_element(cx, el, child_views, leptos::html::caption(cx)),
        "col" => html_element(cx, el, child_views, leptos::html::col(cx)),
        "colgroup" => html_element(cx, el, child_views, leptos::html::colgroup(cx)),
        "table" => html_element(cx, el, child_views, leptos::html::table(cx)),
        "tbody" => html_element(cx, el, child_views, leptos::html::tbody(cx)),
        "td" => html_element(cx, el, child_views, leptos::html::td(cx)),
        "tfoot" => html_element(cx, el, child_views, leptos::html::tfoot(cx)),
        "th" => html_element(cx, el, child_views, leptos::html::th(cx)),
        "thead" => html_element(cx, el, child_views, leptos::html::thead(cx)),
        "tr" => html_element(cx, el, child_views, leptos::html::tr(cx)),
        "button" => html_element(cx, el, child_views, leptos::html::button(cx)),
        "datalist" => html_element(cx, el, child_views, leptos::html::datalist(cx)),
        "fieldset" => html_element(cx, el, child_views, leptos::html::fieldset(cx)),
        "form" => html_element(cx, el, child_views, leptos::html::form(cx)),
        "input" => html_element(cx, el, child_views, leptos::html::input(cx)),
        "label" => html_element(cx, el, child_views, leptos::html::label(cx)),
        "legend" => html_element(cx, el, child_views, leptos::html::legend(cx)),
        "meter" => html_element(cx, el, child_views, leptos::html::meter(cx)),
        "optgroup" => html_element(cx, el, child_views, leptos::html::optgroup(cx)),
        "option" => html_element(cx, el, child_views, leptos::html::option(cx)),
        "output" => html_element(cx, el, child_views, leptos::html::output(cx)),
        "progress" => html_element(cx, el, child_views, leptos::html::progress(cx)),
        "select" => html_element(cx, el, child_views, leptos::html::select(cx)),
        "textarea" => html_element(cx, el, child_views, leptos::html::textarea(cx)),
        "details" => html_element(cx, el, child_views, leptos::html::details(cx)),
        "dialog" => html_element(cx, el, child_views, leptos::html::dialog(cx)),
        "menu" => html_element(cx, el, child_views, leptos::html::menu(cx)),
        "summary" => html_element(cx, el, child_views, leptos::html::summary(cx)),
        "slot" => html_element(cx, el, child_views, leptos::html::slot(cx)),
        "template" => html_element(cx, el, child_views, leptos::html::template(cx)),
        _ => {
            warn!("unknown element {}", el.name);
            ().into_view(cx)
        }
    }
}

fn html_element<Element>(
    cx: Scope,
    element: &html_parser::Element,
    children: Vec<View>,
    mut leptos_el: HtmlElement<Element>,
) -> View
where
    Element: ElementDescriptor + 'static,
{
    if let Some(v) = &element.id {
        leptos_el = leptos_el.id(v.clone());
    }

    for (k, v) in &element.attributes {
        if let Some(v) = v {
            leptos_el = leptos_el.attr(k.clone(), v);
        }
    }

    if !element.classes.is_empty() {
        leptos_el = leptos_el.attr("class", element.classes.join(" "));
    }

    for child in children {
        leptos_el = leptos_el.child(child);
    }

    leptos_el.into_view(cx)
}

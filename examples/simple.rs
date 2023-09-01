use leptos::{ssr::render_to_string, *};
use leptos_mdx::mdx::{Components, Mdx, MdxComponentProps};

fn main() {
    let source = r#"---
title: "Hello, world!"
---

# Hello, world!

This is a **markdown** file with some *content*, but also custom Leptos components!

<custom-title />

<layout>

## subtitle

</layout>

"#;

    let res = render_to_string(|| {
        view! {
            <MyMdx source={source.into()} />
        }
    });

    println!("{}", res);
    // output ->
    //
    // <h1>Hello, world!</h1>
    // <p>This is a <strong>markdown</strong> file with some <em>content</em>, but also custom Leptos components!</p>
    // <h1>Some custom title!</h1>
    // <div class="layout">
    //     <h2>subtitle</h2>
    // </div>
}

#[component]
fn MyMdx(source: String) -> impl IntoView {
    // Register Leptos components...
    let mut components = Components::new();

    // ...without props (easy)
    components.add("custom-title".to_string(), CustomTitle);

    // ...with props (by defining a mapper from `MdxComponentsProps`)
    components.add_props("layout".to_string(), Layout, |props: MdxComponentProps| {
        LayoutProps {
            children: props.children,
        }
    });

    // profit!

    view! {
        <Mdx source={source} components={components} />
    }
}

#[component]
fn CustomTitle() -> impl IntoView {
    view! {
        <h1>Some custom title!</h1>
    }
}

#[component]
fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="layout">
            {children()}
        </div>
    }
}

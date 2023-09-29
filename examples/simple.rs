use leptos::{ssr::render_to_string, *};
use leptos_mdx::mdx::{Components, Mdx, MdxComponentProps};
use std::fs::File;
use std::io::Write;

fn write_to_file(html: &String) -> std::io::Result<()> {
    let mut f = File::create("./examples/test_generated.html")?;
    f.write_all(html.as_bytes())?;
    Ok(())
}


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

    let res = render_to_string(|cx| {
        view! {cx,
            <MyMdx source={source.into()} />
        }
    });
    let _ = write_to_file(&res);
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
fn MyMdx(cx: Scope, source: String) -> impl IntoView {
    // Register Leptos components...
    let mut components = Components::new(cx);

    // ...without props (easy)
    components.add("custom-title".to_string(), CustomTitle);

    // ...with props (by defining a mapper from `MdxComponentsProps`)
    components.add_props("layout".to_string(), Layout, |props: MdxComponentProps| {
        LayoutProps {
            children: props.children,
        }
    });

    // profit!

    view! { cx,
        <Mdx source={source} components={components} />
    }
}

#[component]
fn CustomTitle(cx: Scope) -> impl IntoView {
    view! {cx,
        <h1>Some custom title!</h1>
    }
}

#[component]
fn Layout(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="layout">
            {children(cx)}
        </div>
    }
}

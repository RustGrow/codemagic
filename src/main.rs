use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    // Build cool things ✌️
    rsx! {
        link { rel: "stylesheet", href: asset!("/assets/main.css") }
        img { id: "header", src: asset!("/assets/header.svg") }
        div { id: "links",
            a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
            a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
            a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                "💫 VSCode Extension"
            }
            a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    }
}

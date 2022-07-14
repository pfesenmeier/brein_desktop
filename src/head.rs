use dioxus::prelude::*;


pub static HEAD: Component<()> = |cx| {
    rsx!(cx, head {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" }
            style { [include_str!("./style.css")] }
            style { [include_str!("./twout.css")] }
        })
};


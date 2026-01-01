use dioxus::prelude::*;

pub(crate) fn AhlPatch() -> Element {
    // this is the only fucking way i can bundle these font files
    rsx! {
        style {{
            format!(include_str!("./assets/ahl/ahl.css"),
                asset!("./assets/ahl/mono/tss6AoFBci4C4gvhPXrt3wjT1MqSzhA4t7IIcncBiwKotFCJGR0i.woff2"),
                asset!("./assets/ahl/mono/tss6AoFBci4C4gvhPXrt3wjT1MqSzhA4t7IIcncBiwKotF6JGQ.woff2"),
                asset!("./assets/ahl/mono/tss4AoFBci4C4gvhPXrt3wjT1MqSzhA4t7IIcncBiwKjhFyRHQ.woff2"),
                asset!("./assets/ahl/mono/tss4AoFBci4C4gvhPXrt3wjT1MqSzhA4t7IIcncBiwKthFw.woff2"),
                asset!("./assets/ahl/next/NaPPcYPdHfdVxJw0IfIP0lvYFqijb-UxCtm5_wdGseiMr3a-oXZ-.woff2"),
                asset!("./assets/ahl/next/NaPPcYPdHfdVxJw0IfIP0lvYFqijb-UxCtm5_wdGseiMr3i-oQ.woff2"),
                asset!("./assets/ahl/next/NaPNcYPdHfdVxJw0IfIP0lvYFqijb-UxCtm5_wdGseiHn3qmpQ.woff2"),
                asset!("./assets/ahl/next/NaPNcYPdHfdVxJw0IfIP0lvYFqijb-UxCtm5_wdGseiJn3o.woff2"),
            )
        }}
    }
}

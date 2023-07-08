mod emulator;

use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <div> { "Hello, World!" } </div>
        }
    });
}

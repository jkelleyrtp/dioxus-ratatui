use dioxus::prelude::*;

fn main() {
    dioxus_ratatui::launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    use_future(move || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            count += 1;
        }
    });

    rsx! {
        div { class: "flex flex-row",
            "dx run -i"
            "rust 1.70 | stable | dx 0.5.2"
        }
    }
}

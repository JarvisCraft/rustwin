use std::time::Duration;

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>Rustwin</h1>

        <h2>Time since Rustwin</h2>
        <Timer />

        <h2>The Crabs</h2>
        <TheCrabs />
    }
}

#[component]
fn Timer() -> impl IntoView {
    // Rust Release 1.0: https://github.com/rust-lang/rust/releases/tag/1.0.0
    // e.g. commit `a59de37e99060162a2674e3ff45409ac73595c0e`
    const RUST_1_0_INIT_TIMESTAMP: u64 = 1431572853 * 1000;

    let timestamp = leptos_use::use_timestamp();
    let win_duration = move || {
        let duration = Duration::from_millis(timestamp.get() as u64 - RUST_1_0_INIT_TIMESTAMP);
        humantime::format_duration(duration).to_string()
    };
    view! { <div>"Time since rustwin: " {win_duration}</div> }
}

#[derive(Copy, Clone, Debug)]
struct Crab {
    message: &'static str,
}
impl Crab {
    const MESSAGES: [&'static str; 5] = [
        "Hi there!",
        "Hello!",
        "It's me",
        "I'm the Crab",
        "Check This Week In Rust",
    ];

    pub fn random() -> Self {
        use rand::{rng, seq::IndexedRandom};
        let message = Self::MESSAGES.choose(&mut rng()).unwrap();
        Self { message }
    }
}

#[component]
fn TheCrabs() -> impl IntoView {
    let mut next_id = 3u64;
    let initial_crabs = (0..next_id)
        .map(|id| (id, ArcRwSignal::new(Crab::random())))
        .collect::<Vec<_>>();

    let (crabs, set_crabs) = signal(initial_crabs);
    let add_crab = move |_| {
        let signal = ArcRwSignal::new(Crab::random());
        set_crabs.update(move |crabs| crabs.push((next_id, signal)));
        next_id += 1;
    };

    view! {
        <div>
            <button on:click=add_crab>Add Crab</button>
            <ul>
                <For
                    each=move || crabs.get()
                    key=|(id, _)| *id
                    children=move |(id, crab)| {
                        view! {
                            <li>
                                <q>{crab.get().message}</q>
                                <br />
                                <Crab />
                                <br />
                                <button on:click=move |_| {
                                    set_crabs.write().retain(|(checked_id, _)| *checked_id != id)
                                }>Remove crab</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

/// A spinning Crab.
#[component]
fn Crab() -> impl IntoView {
    view! {
        <img
            src="images/rustacean-flat-noshadow.svg"
            alt="Ferris the Crab"
            width="10%"
            class="crab"
        />
    }
}

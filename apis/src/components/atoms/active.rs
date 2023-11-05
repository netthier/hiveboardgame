use crate::common::svg_pos::SvgPos;
use crate::providers::game_state::GameStateSignal;
use hive_lib::position::Position;
use leptos::*;

#[component]
pub fn Active(
    position: Position,
    #[prop(into)] level: MaybeSignal<usize>,
    #[prop(optional)] extend_tw_classes: &'static str,
) -> impl IntoView {
    let center = move || SvgPos::center_for_level(position, level());
    let transform = move || format!("translate({},{})", center().0, center().1);
    let mut game_state_signal = expect_context::<GameStateSignal>();

    let onclick = move |_| {
        game_state_signal.reset();
    };

    view! {
        <g on:click=onclick class=format!("{extend_tw_classes}")>
            <g id="Active" transform=transform>
                <use_
                    href="#active"
                    transform="scale(0.56, 0.56) translate(-46.608, -52.083)"
                ></use_>
            </g>
        </g>
    }
}


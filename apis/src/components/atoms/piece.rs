use crate::common::{piece_type::PieceType, svg_pos::SvgPos};
use crate::providers::game_state::GameStateSignal;
use hive_lib::{bug::Bug, piece::Piece, position::Position};
use leptos::logging::log;
use leptos::*;

#[component]
pub fn Piece(
    // WARN piece and position are currently not used in closures and that is temporarily ok because they are passed in as not signals
    #[prop(into)] piece: MaybeSignal<Piece>,
    #[prop(into)] position: MaybeSignal<Position>,
    #[prop(into)] level: MaybeSignal<usize>,
    #[prop(optional, into)] piece_type: PieceType,
) -> impl IntoView {
    let center = move || SvgPos::center_for_level(position(), level());
    let transform = move || format!("translate({},{})", center().0, center().1);
    //IMPORTANT drop-shadow-b drop-shadow-w leave this comment for TW
    let mut filter = String::from("drop-shadow-");
    filter.push_str(&piece.get().color().to_string());
    if piece_type == PieceType::Inactive {
        filter.push_str(" sepia");
    }
    let color = piece.get().color().to_string();
    let bug = piece.get().bug().to_string();
    let order = piece.get().order().to_string();

    let mut dot_color = String::from(" color: #");
    dot_color.push_str(match piece.get().bug() {
        Bug::Ant => "3574a5",
        Bug::Beetle => "7a4fab",
        Bug::Grasshopper => "3f9b3a",
        Bug::Spider => "993c1e",
        _ => "FF0000",
    });

    let mut game_state_signal = expect_context::<GameStateSignal>();

    let onclick = move |_| match piece_type {
        PieceType::Board => {
            log!("Board piece");
            game_state_signal.show_moves(piece.get(), position.get());
        }
        PieceType::Reserve => {
            log!("Reserve piece");
            game_state_signal.show_spawns(piece.get(), position.get());
        }
        PieceType::Spawn => {
            log!("Spawning piece {}", piece.get());
            game_state_signal.play_active_piece();
        }
        _ => log!("Piece is {}", piece_type),
    };

    view! {
        <g on:click=onclick class=filter style=dot_color>
            <g transform=transform>
                <use_
                    href=format!("#{}", color)
                    transform="scale(0.56, 0.56) translate(-45, -50)"
                ></use_>
                <use_
                    href=format!("#{}", bug)
                    transform="scale(0.56, 0.56) translate(-50, -45)"
                ></use_>
                <use_
                    href=format!("#{}", order)
                    transform="scale(0.56, 0.56) translate(-45, -50)"
                ></use_>
            </g>
        </g>
    }
}


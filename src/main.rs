use std::fmt;

use leptos::{*};

fn main() {
    mount_to_body(|| view! { <Board/> })
}

#[derive(Clone, Debug,Copy, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Red
}

#[derive(Clone, Debug, Copy, PartialEq)]
enum Player {
    A,
    B
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::A => write!(f, "A"),
            Player::B => write!(f, "B"),
        }
    }
}

#[derive(Debug,Clone)]
struct Cell {
    played_by: RwSignal<Option<Player>>,
    color: RwSignal<Color>,
    number: u8    
}


fn get_winner(board: Vec<Cell>) -> Option<Player> {
    let winning_placements = [
        //lines
        [0,1,2],
        [3,4,5],
        [6,7,8],
        //columns
        [0,3,6],
        [1,4,7],
        [2,5,8],
        //diagonals
        [0,4,8],
        [6,4,2]
    ];
    
winning_placements.into_iter().find_map(move |combination| {
            let a = board[combination[0]].played_by.get();
            let b = board[combination[1]].played_by.get();
            let c = board[combination[2]].played_by.get();

            // logging::log!("a: {:?}", a);
            // logging::log!("b: {:?}", b);
            // logging::log!("c: {:?}", c);

            if a == b  && b == c && a.is_some() {
                return Some(a.unwrap())
            }

            None
           
        })
}

fn create_board() -> Vec<Cell> {
(1..=9).map(|number| Cell{
        played_by: create_rw_signal(None),
        color: create_rw_signal(Color::White),
        number
    }).collect::<Vec<Cell>>()    
}

fn clear_board(board: Vec<Cell>) {
    board.into_iter().for_each(|cell| {
        cell.played_by.set(None);
        cell.color.set(Color::White)
    })
} 

#[component]
fn Board() -> impl IntoView {
    let (board , _)= create_signal(create_board());
       
    let game_finished = move || !board.get().into_iter().any(|x| x.color.get() == Color::White);

    let winner = move || get_winner(board.get());

    let (player, set_player) = create_signal(Player::A);

    view! {
        <Show when=move || !winner().is_some()>
            <p>"player: " {move || player.get().to_string()}</p>
        </Show>
        <Show when=move || winner().is_some()>
            <p>"Game won by player " {winner().unwrap().to_string()}</p>
        </Show>
        <Show when=move || winner().is_some() || game_finished()>
            <button
                class="border border-black"
                on:click=move |_| {
                    clear_board(board.get());
                    set_player(Player::A)
                }
            >

                "New game"
            </button>
        </Show>
        <div class="grid grid-cols-3 w-[300px] h-[300px]">
            <For each=board key=|state| state.number.clone() let:child>
                <button
                    disabled=move || winner().is_some() || child.color.get() != Color::White
                    on:click=move |_| {
                        child.played_by.set(Some(player.get()));
                        child
                            .color
                            .set(
                                match player.get() {
                                    Player::A => Color::Blue,
                                    Player::B => Color::Red,
                                },
                            );
                        set_player(
                            match player.get() {
                                Player::A => Player::B,
                                Player::B => Player::A,
                            },
                        );
                        logging::log!("{:?}", player.get())
                    }

                    class="
                    border border-black
                    w-[100px] h-[100px]
                    flex items-center justify-center
                    "
                    class=("bg-white", move || child.color.get() == Color::White)
                    class=("bg-red-300", move || child.color.get() == Color::Red)
                    class=("bg-blue-200", move || child.color.get() == Color::Blue)
                ></button>
            </For>
        </div>
    }
    }

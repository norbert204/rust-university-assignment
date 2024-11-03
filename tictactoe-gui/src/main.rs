use gloo::timers::callback::Timeout;
use rand::Rng;
use yew::prelude::*;

pub fn check_winner(cells: &tictactoe::grid::Grid) -> Option<tictactoe::grid::BoardGridCell> {
    fn check_series(cells: &tictactoe::grid::Grid, starting_pos: usize, offset: usize, needed_matches: i32) -> Option<tictactoe::grid::BoardGridCell> {
        let value = cells[starting_pos];

        if value == tictactoe::grid::BoardGridCell::None {
            return None;
        }

        for i in 0..needed_matches {
            if cells[starting_pos + i as usize * offset] != value {
                return None;
            }
        }

        return Some(value);
    }

    if cells.iter().all(|x| *x != tictactoe::grid::BoardGridCell::None) {
        return Some(tictactoe::grid::BoardGridCell::None);
    }

    let starts_and_offsets = [
        ( 0, 1 ),
        ( 3, 1 ),
        ( 6, 1 ),
        ( 0, 3 ),
        ( 1, 3 ),
        ( 2, 3 ),
        ( 0, 4 ),
        ( 2, 2 ),
    ];

    for (start, offset) in starts_and_offsets {
        let result = check_series(&cells, start, offset, 3);

        if let Some(x) = result {
            return Some(x);
        }
    }

    None
}

#[derive(PartialEq, Clone)]
struct BoardContext {
    grid: tictactoe::grid::Grid,
    player_step: bool,
    winner: Option<tictactoe::grid::BoardGridCell>
}

impl Reducible for BoardContext {
    type Action = (tictactoe::grid::Grid, bool, Option<tictactoe::grid::BoardGridCell>);

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        BoardContext {
            grid: action.0,
            player_step: action.1,
            winner: action.2,
        }.into()
    }
}

#[function_component]
fn Board() -> Html {
    let context = use_context::<UseReducerHandle<BoardContext>>().unwrap();

    context.grid
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let board = context.clone();
            
            let disabled = !board.player_step;

            let data = match x {
                tictactoe::grid::BoardGridCell::None => "none",
                tictactoe::grid::BoardGridCell::Circle => "circle",
                tictactoe::grid::BoardGridCell::Cross => "cross"
            };

            let img = match x {
                tictactoe::grid::BoardGridCell::None => None,
                tictactoe::grid::BoardGridCell::Circle => Some("img/circle.svg"),
                tictactoe::grid::BoardGridCell::Cross => Some("img/cross.svg")
            };

            let callback = Callback::from(move |_| {
                let mut grid = board.grid.clone();

                if grid[i] != tictactoe::grid::BoardGridCell::None || board.winner.is_some() {
                    return;
                }

                grid[i] = tictactoe::grid::BoardGridCell::Circle;

                let winner = check_winner(&grid);

                board.dispatch((grid, false, winner));

                if winner.is_some() {
                    return;
                }

                let board = board.clone();

                Timeout::new(3000, move || {
                    let mut rng = rand::thread_rng();
                    
                    loop {
                        let index = rng.gen_range(0..grid.len());

                        if grid[index] == tictactoe::grid::BoardGridCell::None {
                            grid[index] = tictactoe::grid::BoardGridCell::Cross;
                            break;
                        }
                    }
                    
                    let winner = check_winner(&grid);

                    board.dispatch((grid, true, winner));
                })
                .forget();
            });

            html! {
                <button data-cell={data} onclick={callback} disabled={disabled}>
                    if let Some(img) = img {
                        <img src={img} />
                    }
                </button>
            }
        })
        .collect()
}

#[function_component]
fn Replay() -> Html {
    let context = use_context::<UseReducerHandle<BoardContext>>().unwrap();

    let onclick = Callback::from(move |_| {
        context.dispatch(([tictactoe::grid::BoardGridCell::None; 9], true, None));
    });

    html! {
        <button {onclick} class="replay-button">
            { "Replay" }
        </button>
    }
}

#[function_component]
fn App() -> Html {
    let board = use_reducer(|| {
        BoardContext {
            grid: [tictactoe::grid::BoardGridCell::None; 9],
            player_step: true,
            winner: None,
        }
    });

    let (have_winner, class, winner_text ) = match board.winner {
        None => (false, "", ""),
        Some(x) => match x {
            tictactoe::grid::BoardGridCell::None => (true, "", "Draw"),
            tictactoe::grid::BoardGridCell::Circle => (true, "circle", "You win"),
            tictactoe::grid::BoardGridCell::Cross => (true, "cross", "Opponent win"),
        }
    };

    let player_step = board.player_step;

    html! {
        <ContextProvider<UseReducerHandle<BoardContext>> context={board}>
            <div class={classes!["container", class]}>
                <div class="info">
                    if have_winner {
                        { winner_text }
                    } else {
                        if player_step {
                            { "Your turn" }
                        } else {
                            { "Opponents turn" }
                            <span class="loading"></span>
                        }
                    }
                </div>
                <div class="board">
                    <Board />
                </div>
                if have_winner {
                    <Replay />
                }
            </div>
        </ContextProvider<UseReducerHandle<BoardContext>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

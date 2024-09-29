mod chars;

use std::{error::Error, io::Write, process::exit};
use tictactoe::{BoardGridCell, TicTacToe};

fn clear_screen() {
    print!("{}c", 27 as char);
}

// So ugly!
fn print_field(cells: &[BoardGridCell; 9]) {
    let mut field = format!(
        "{btl}{bh}{bhb}{bh}{bhb}{bh}{btr}\n\
        {bv} 1 {bv} 2 {bv} 3 {bv}\n\
        {bvr}{bh}{bc}{bh}{bc}{bh}{bvl}\n\
        {bv} 4 {bv} 5 {bv} 6 {bv}\n\
        {bvr}{bh}{bc}{bh}{bc}{bh}{bvl}\n\
        {bv} 7 {bv} 8 {bv} 9 {bv}\n\
        {bbl}{bh}{bht}{bh}{bht}{bh}{bbr}",
        bh = chars::BORDER_HORIZONTAL.repeat(3),
        bv = chars::BORDER_VERTICAL,
        btl = chars::BORDER_TOP_LEFT,
        btr = chars::BORDER_TOP_RIGHT,
        bbl = chars::BORDER_BOTTOM_LEFT,
        bbr = chars::BORDER_BOTTOM_RIGHT,
        bvr = chars::BORDER_VERTICAL_RIGHT,
        bvl = chars::BORDER_VERTICAL_LEFT,
        bhb = chars::BORDER_HORIZONTAL_BOTTOM,
        bht = chars::BORDER_HORIZONTAL_TOP,
        bc = chars::BORDER_CROSS,
    );

    for (i, cell) in cells.iter().enumerate() {
        let cell_index = (i + 1) as i32;

        field = field.replace(
            cell_index.to_string().as_str(),
            to_board_string(cell, i).as_str(),
        );
    }

    println!("{}", field);
    std::io::stdout().flush().unwrap(); // Needed, otherwise the text won't show up.
}

fn player_step(game: &mut TicTacToe) -> Result<(), Box<dyn Error>> {
    print!("\x1b[93mSelect cell [1-9] > \x1b[0m");
    std::io::stdout().flush().unwrap(); // Needed, otherwise the text won't show up.

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let input = input.trim();

    let index = input.parse::<i32>();

    if let Err(_) = index {
        return Err(format!("'{}' is not a number!", input).into());
    }

    let index = index.unwrap();

    game.player_step(index - 1)?;

    Ok(())
}

fn win_check(game: &TicTacToe) {
    let winner = game.check_winner();

    if let Some(x) = winner {
        println!("{} wins!", x.to_string());
        exit(0);
    }
}


fn to_board_string(cell: &BoardGridCell, index: usize) -> String {
    match cell {
        BoardGridCell::None => (index + 1).to_string(),
        BoardGridCell::Cross => "X".to_owned(),
        BoardGridCell::Circle => "O".to_owned(),
    }
}

fn main() {
    let mut game = tictactoe::TicTacToe::default();

    loop {
        clear_screen();
        print_field(&game.cells());

        let result = player_step(&mut game);

        if let Err(e) = result {
            println!("{}", e);

            std::thread::sleep(std::time::Duration::from_secs(2));

            continue;
        }

        clear_screen();
        print_field(&game.cells());

        win_check(&game);

        clear_screen();
        print_field(&game.cells());

        println!("Enemy is picking...");

        std::thread::sleep(std::time::Duration::from_secs(1));

        game.enemy_step();

        clear_screen();
        print_field(&game.cells());

        win_check(&game);
    }
}

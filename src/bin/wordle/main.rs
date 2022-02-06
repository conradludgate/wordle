mod game;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let game = game::Game::new()?;
    if let Some(share) = game.start()? {
        println!("{}", share);
    }

    Ok(())
}

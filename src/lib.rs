use std::io;

use console::{Console, Event};
use logic::Game;
use ui::{Scene, Transition, scenes};

mod console;
mod logic;
mod ui;

pub fn run() -> Result<(), io::Error> {
    let mut console = Console::new(ui::CONSOLE_WIDTH, ui::CONSOLE_HEIGHT, "Goblin Castle")?;
    let mut game = Game::new();
    let mut stack: Vec<Box<dyn Scene>> = vec![];
    let mut scene: Box<dyn Scene> = Box::new(scenes::StartScreen);

    loop {
        console.clear();
        for scene in &stack {
            scene.render(&game, &mut console);
        }
        scene.render(&game, &mut console);
        console.display()?;

        let event = console.read_event()?;
        match event {
            Event::Abort => break,
            event => match scene.handle_event(&mut game, event) {
                Transition::Okay => {}
                Transition::Beep => console.alert()?,
                Transition::Switch(next) => scene = next,
                Transition::Push(next) => {
                    stack.push(scene);
                    scene = next;
                }
                Transition::Pop => {
                    if let Some(prev) = stack.pop() {
                        scene = prev;
                    } else {
                        break;
                    }
                }
            },
        }
    }

    Ok(())
}

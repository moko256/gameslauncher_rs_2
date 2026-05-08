mod app;

use std::{thread::sleep, time::Duration};

use app_common::features::launcher::launch_item_repository::LaunchItemRepository;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();

    match result {
        Ok(stop_details) => {
            match stop_details.stop_reason {
                app::StopReason::LaunchApp => {
                    if let Some(item) = stop_details.selected_item {
                        LaunchItemRepository::launch_item(&item);
                    }

                    sleep(Duration::from_secs(5));
                }
                _ => {
                    // Do nothing
                }
            }

            Ok(())
        }
        Err(err) => Err(err),
    }
}

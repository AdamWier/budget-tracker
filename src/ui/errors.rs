use std::panic;

use anyhow::Result;
use color_eyre::{config::HookBuilder, eyre};

use crate::ui;

pub fn install_hooks() -> Result<()> {
    let (color_eyre_panic_hook, color_eyre_hook) = HookBuilder::default().into_hooks();

    let standard_panic_hook = color_eyre_panic_hook.into_panic_hook();

    panic::set_hook(Box::new(move |panic_info| {
        ui::wrapper::restore().unwrap();
        standard_panic_hook(panic_info);
    }));

    let eyre_hook = color_eyre_hook.into_eyre_hook();

    eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            ui::wrapper::restore().unwrap();
            eyre_hook(error)
        },
    ))?;

    Ok(())
}

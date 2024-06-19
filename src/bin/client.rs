slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = ClientUI::new()?;

    ui.run()
}

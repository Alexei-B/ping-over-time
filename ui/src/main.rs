use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Label, Flex};

pub mod plot;
pub mod timeseries;

fn build_ui() -> impl Widget<()> {
    Flex::row()
        .with_flex_child(
            Flex::column()
                .with_flex_child(Label::new("top left"), 1.0),
            1.0)
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui)).launch(())?;
    Ok(())
}

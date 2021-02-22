use std::io;
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
    Terminal,
};

pub struct Plot;

impl Plot {
    pub fn run() -> Result<(), io::Error> {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        loop {
            terminal
                .draw(|f| {
                    let size = f.size();
                    let datasets = vec![Dataset::default()
                        .name("google.com")
                        .marker(symbols::Marker::Dot)
                        .graph_type(GraphType::Scatter)
                        .style(Style::default().fg(Color::Cyan))
                        .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)])];

                    let chart = Chart::new(datasets)
                        .block(Block::default().title("Pings"))
                        .x_axis(
                            Axis::default()
                                .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                                .style(Style::default().fg(Color::White))
                                .bounds([0.0, 10.0])
                                .labels(
                                    ["0.0", "5.0", "10.0"]
                                        .iter()
                                        .cloned()
                                        .map(Span::from)
                                        .collect(),
                                ),
                        )
                        .y_axis(
                            Axis::default()
                                .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
                                .style(Style::default().fg(Color::White))
                                .bounds([0.0, 10.0])
                                .labels(
                                    ["0.0", "5.0", "10.0"]
                                        .iter()
                                        .cloned()
                                        .map(Span::from)
                                        .collect(),
                                ),
                        );

                    f.render_widget(chart, size);
                })
                .unwrap();
        }
    }
}

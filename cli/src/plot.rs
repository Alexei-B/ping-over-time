use std::{convert::TryInto, io};
use pot_rpc::{Ping, PingsRequest, PingsServiceClient};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
    Terminal,
};
use std::time::{Duration, SystemTime};

pub struct Plot;

impl Plot {
    pub async fn run() -> Result<(), io::Error> {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        let mut client = PingsServiceClient::connect("http://[::1]:50051").await.unwrap();
        terminal.clear()?;

        loop {
            let now = SystemTime::now();
            let pings: Vec<Ping> = client.get_pings(tonic::Request::new(PingsRequest {
                address: String::from("172.217.169.14"),
                since: Some((now - Duration::from_secs(60)).into()),
                until: None
            })).await.unwrap().into_inner().pings;

            let now = SystemTime::now();
            let data: Vec<(f64, f64)> = pings.into_iter().map(|p| (
                -(now.duration_since(p.time.unwrap().into()).unwrap().as_millis() as f64)/1000.0,
                Duration::from(p.duration.unwrap().try_into().unwrap()).as_millis() as f64
            )).collect();

            terminal
                .draw(|f| {
                    let size = f.size();

                    let datasets = vec![Dataset::default()
                        .name("www.google.com")
                        .marker(symbols::Marker::Braille)
                        .graph_type(GraphType::Line)
                        .style(Style::default().fg(Color::Cyan))
                        .data(data.as_slice())];

                    let chart = Chart::new(datasets)
                        .block(Block::default().title("Pings"))
                        .x_axis(
                            Axis::default()
                                .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                                .style(Style::default().fg(Color::White))
                                .bounds([-60.0, 0.0])
                                .labels(
                                    ["-60.0", "-30.0", "0.0"]
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
                                .bounds([0.0, 100.0])
                                .labels(
                                    ["0.0", "50.0", "100.0"]
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

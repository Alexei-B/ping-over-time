use crate::timeseries::Timeseries;
use druid::kurbo::{Line, Point};
use druid::piet::Color;
use druid::widget::prelude::*;
use std::cmp::Ordering::Equal;

pub struct Plot {
    pub timeseries: Timeseries,
}

impl Widget<Timeseries> for Plot {
    fn paint(&mut self, ctx: &mut PaintCtx<'_, '_, '_>, data: &Timeseries, env: &Env) {
        let min_time = self
            .timeseries
            .points()
            .map(|p| p.0.timestamp())
            .min()
            .unwrap();
        let max_time = self
            .timeseries
            .points()
            .map(|p| p.0.timestamp())
            .max()
            .unwrap();

        let min_value = self
            .timeseries
            .points()
            .map(|p| p.1)
            .min_by(|l, r| l.partial_cmp(r).unwrap_or(Equal))
            .unwrap();
        let max_value = self
            .timeseries
            .points()
            .map(|p| p.1)
            .max_by(|l, r| r.partial_cmp(l).unwrap_or(Equal))
            .unwrap();

        let min_time = if min_time < 0 { min_time } else { 0 };
        let min_value = if min_value < 0.0 { min_value } else { 0.0 };

        let time_range = max_time - min_time;
        let value_range = max_value - min_value;

        let scaled = self
            .timeseries
            .points()
            .map(|p| {
                (
                    (p.0.timestamp() - min_time) as f64 / (time_range as f64),
                    (p.1 - min_value) / value_range,
                )
            })
            .collect::<Vec<(f64, f64)>>();

        let mut lines = Vec::new();
        for i in 1..scaled.len() {
            lines.push(Line::new(
                Point::new(scaled[i - 1].0, scaled[i - 1].1),
                Point::new(scaled[i].0, scaled[i].1),
            ))
        }

        for line in lines {
            ctx.stroke(line, &Color::AQUA, 1.0);
        }
    }

    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, _data: &mut Timeseries, _env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                dbg!(mouse);
            }
            _ => (),
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Timeseries,
        env: &Env,
    ) -> Size {
        bc.max()
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &Timeseries,
        _data: &Timeseries,
        _env: &Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Timeseries,
        _env: &Env,
    ) {
    }
}

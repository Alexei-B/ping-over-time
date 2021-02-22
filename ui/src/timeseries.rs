use std::slice::Iter;
use druid::Data;
use chrono::{ DateTime, Utc };

#[derive(Clone, Debug)]
pub struct Timeseries {
    data: Vec<(DateTime<Utc>, f64)>
}

impl Timeseries {
    pub fn new() -> Self {
        Timeseries { data: Vec::new() }
    }

    pub fn points(&self) -> Iter<(DateTime<Utc>, f64)> {
        self.data.iter()
    }
}

impl Data for Timeseries {
    fn same(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }

        for i in 0..self.data.len() {
            if self.data[i].0 != other.data[i].0 || self.data[i].1 != other.data[i].1 {
                return false;
            }
        }

        true
    }
}

pub trait Average where Self: Iterator<Item=f64> {
    fn average(self) -> Option<f64>;
}

impl<I: Iterator<Item=f64>> Average for I  {
    fn average(mut self) -> Option<f64> {
        let mut val = self.next()?;
        let mut n = 1;
        for v in self {
            val = val + v;
            n += 1;
        }
        Some(val/(n as f64))
    }
}

pub trait Median where Self: Iterator<Item=f64> {
    fn median(self) -> Option<f64>;
}

impl<I: Iterator<Item=f64>> Median for I {
    fn median(self) -> Option<f64> {
        let mut values: Vec<f64> = self.collect();
        if values.is_empty() {
            return None;
        }

        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let len = values.len();
        if len % 2 == 1 {
            Some(values[len / 2])
        } else {
            Some((values[len / 2 - 1] + values[len / 2]) / 2.0)
        }
    }
}

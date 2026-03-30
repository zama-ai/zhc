use crate::small::SmallMap;
use std::{fmt::Display, hash::Hash};

pub struct Plot<X: Eq, Y>(SmallMap<X, Y>);

impl<X: Eq + Hash + Clone, Y: Clone> Plot<X, Y> {
    pub fn empty() -> Self {
        Plot(SmallMap::new())
    }

    pub fn insert(&mut self, x: &X, y: &Y) {
        self.0.insert(x.clone(), y.clone());
    }
}

impl<X, Y> Display for Plot<X, Y>
where
    X: Eq + Hash + Ord + Display + Clone,
    Y: Copy + Into<f64>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const WIDTH: usize = 60;

        if self.0.is_empty() {
            return Ok(());
        }

        // Sort by X (increasing)
        let mut entries: Vec<_> = self
            .0
            .iter()
            .map(|(x, y)| (x.clone(), (*y).into()))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));

        // Scale Y to [0, WIDTH-1]
        let min_y = entries
            .iter()
            .map(|(_, y)| *y)
            .fold(f64::INFINITY, f64::min);
        let max_y = entries
            .iter()
            .map(|(_, y)| *y)
            .fold(f64::NEG_INFINITY, f64::max);
        let range = (max_y - min_y).max(1.0);

        let labels: Vec<_> = entries.iter().map(|(x, _)| x.to_string()).collect();
        let max_label_w = labels.iter().map(|s| s.len()).max().unwrap_or(0);

        // Gradient: ░ ▒ ▓ ●
        for (i, (_, y)) in entries.iter().enumerate() {
            let offset = ((y - min_y) / range * (WIDTH - 1) as f64) as usize;
            let mut line = vec![' '; WIDTH];
            // Fill with gradient
            for j in 0..=offset {
                line[j] = match offset - j {
                    0 => '●',
                    1 => '▓',
                    2 => '▒',
                    _ => '░',
                };
            }
            let line_str: String = line.into_iter().collect();
            writeln!(f, "{:>w$} │{}", labels[i], line_str, w = max_label_w)?;
        }

        Ok(())
    }
}

use crate::{Dumpable, small::SmallMap};
use std::{fmt::Display, hash::Hash};

pub struct Histogram<Bin: Hash + Eq + Clone>(SmallMap<Bin, u32>);

impl<Bin: Hash + Eq + Clone> Histogram<Bin> {
    pub fn empty() -> Self {
        Histogram(SmallMap::new())
    }

    pub fn count(&mut self, b: &Bin) {
        if !self.0.contains_key(b) {
            self.0.insert(b.clone(), 0);
        }
        *self.0.get_mut(b).unwrap() += 1;
    }
}

impl<Bin: Hash + Eq + Clone + Display + Ord> Display for Histogram<Bin> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const BAR_WIDTH: usize = 65;

        if self.0.is_empty() {
            return Ok(());
        }

        let mut entries: Vec<_> = self.0.iter().collect();
        entries.sort_by_key(|(bin, _)| *bin);

        let max_count = entries
            .iter()
            .map(|(_, c)| **c)
            .max()
            .unwrap_or(1u32)
            .max(1);
        let labels: Vec<_> = entries.iter().map(|(b, _)| b.to_string()).collect();
        let max_label_width = labels.iter().map(|s| s.len()).max().unwrap_or(0);

        // Gradient: ░ ▒ ▓ ●
        for (i, (_, count)) in entries.iter().enumerate() {
            let bar_len = (**count as usize * BAR_WIDTH) / max_count as usize;
            let bar: String = (0..bar_len)
                .map(|j| match bar_len - 1 - j {
                    0 => '●',
                    1 => '▓',
                    2 => '▒',
                    _ => '░',
                })
                .collect();
            writeln!(
                f,
                "{:>width$} │{} ({})",
                labels[i],
                bar,
                count,
                width = max_label_width
            )?;
        }

        Ok(())
    }
}

impl<Bin: Hash + Eq + Clone + Display + Ord> Dumpable for Histogram<Bin> {
    fn dump_to_string(&self) -> String {
        self.to_string()
    }
}

use std::collections::HashMap;

pub struct Topology {
    nodes: Vec<Vec<usize>>,
    cpu_node: HashMap<usize, usize>,
}

impl Topology {
    pub fn detect() -> Self {
        Self::from_sysfs().unwrap_or_else(Self::single_node)
    }

    pub fn n_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn node_of(&self, cpu: usize) -> usize {
        self.cpu_node.get(&cpu).copied().unwrap_or(0)
    }

    pub fn representative_cpu(&self, node: usize) -> usize {
        self.nodes[node][0]
    }

    fn single_node() -> Self {
        let n = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        let cpus: Vec<usize> = (0..n).collect();
        let cpu_node = cpus.iter().map(|&c| (c, 0)).collect();
        Topology { nodes: vec![cpus], cpu_node }
    }

    fn from_sysfs() -> Option<Self> {
        const BASE: &str = "/sys/devices/system/node";
        let mut ids: Vec<usize> = std::fs::read_dir(BASE)
            .ok()?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                e.file_name()
                    .to_str()?
                    .strip_prefix("node")?
                    .parse::<usize>()
                    .ok()
            })
            .collect();
        ids.sort_unstable();
        if ids.is_empty() {
            return None;
        }

        let mut nodes = Vec::with_capacity(ids.len());
        let mut cpu_node = HashMap::new();
        for (dense, id) in ids.into_iter().enumerate() {
            let list = std::fs::read_to_string(format!("{BASE}/node{id}/cpulist")).ok()?;
            let cpus = parse_cpulist(&list);
            for &c in &cpus {
                cpu_node.insert(c, dense);
            }
            nodes.push(cpus);
        }
        Some(Topology { nodes, cpu_node })
    }
}

pub fn run_on_cpu<T: Send>(cpu: usize, f: impl FnOnce() -> T + Send) -> T {
    std::thread::scope(|s| {
        s.spawn(|| {
            core_affinity::set_for_current(core_affinity::CoreId { id: cpu });
            f()
        })
        .join()
        .expect("NUMA init thread panicked")
    })
}

fn parse_cpulist(s: &str) -> Vec<usize> {
    let mut cpus = Vec::new();
    for part in s.trim().split(',').filter(|p| !p.is_empty()) {
        match part.split_once('-') {
            Some((a, b)) => {
                if let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) {
                    cpus.extend(a..=b);
                }
            }
            None => {
                if let Ok(c) = part.parse::<usize>() {
                    cpus.push(c);
                }
            }
        }
    }
    cpus
}

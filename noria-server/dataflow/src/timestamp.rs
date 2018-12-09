
use std::collections::BTreeMap;
use prelude::*;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Time(u64);

#[derive(Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TimestampAssigner(u64);
impl TimestampAssigner {
    pub fn assign(&mut self) -> TimeComponent {
        self.0 += 1;
        TimeComponent {
            time: Time(self.0 - 1),
            path: Path(0),
        }
    }
}
impl Clone for TimestampAssigner {
    fn clone(&self) -> TimestampAssigner {
        assert_eq!(self.0, 0);
        TimestampAssigner(0)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Path(u64);

#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TimeComponent {
    pub path: Path,
    pub time: Time,
}

/// Maps incoming paths at a node to outgoing paths at a node.
#[derive(Clone, Serialize, Deserialize)]
struct PathMap(Option<Vec<Vec<Path>>>);
impl PathMap {
    pub fn lookup(&self, ancestor: usize, incoming: Path) -> Path {
        match self.0 {
            Some(ref v) => v[ancestor][incoming.0 as usize],
            None => incoming,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VectorTime {
    components: Vec<Time>,
}
impl VectorTime {
    /// Create a new empty `VectorTime`
    pub fn new() -> Self {
        Self { components: Vec::new() }
    }

    fn with_length(len: usize) -> Self {
        Self { components: vec![Time(0); len] }
    }

    /// Add an additional component to the `VectorTime`.
    pub fn extend(&mut self, component: TimeComponent) {
        let index = component.path.0 as usize;
        assert_eq!(self.components.len() + 1, index);
        self.components.push(component.time);
    }

    /// Advance a single time component.
    pub fn advance(&mut self, component: TimeComponent) {
        let index = component.path.0 as usize;
        assert_eq!(self.components[index].0 + 1, component.time.0);
        self.components[index] = component.time;
    }
}

/// Encapsulates all time related state for a node.
#[derive(Clone, Serialize, Deserialize)]
pub struct NodeTimeState {
    time: VectorTime,
    paths: PathMap,
    base_paths: Vec<((NodeIndex, usize), Vec<Path>)>
}
impl NodeTimeState {
    /// Checks whether this node may contain split updates.
    pub fn is_consistent(&self) -> bool {
        for (_, ref paths) in &self.base_paths {
            let time = self.time.components[paths[0].0 as usize];
            for i in 1..paths.len() {
                if self.time.components[paths[i].0 as usize] != time {
                    return false;
                }
            }
        }
        return true;
    }

    /// Updates the internal state, and returns a new time component that should be emitted by this
    /// node.
    pub fn process_update(&mut self, ancestor: usize, time_component: TimeComponent) -> TimeComponent {
        self.time.advance(time_component);
        TimeComponent {
            path: self.paths.lookup(ancestor, time_component.path),
            ..time_component
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct PathAssignments {
    /// Map from node to the path assignments for each shard of it.
    assignments: BTreeMap<NodeIndex, Vec<PathMap>>,
}
impl PathAssignments {
    pub fn add_node(&mut self, node: NodeIndex, shards: usize, graph: &Graph) {
        unimplemented!()
    }

    pub fn make_node_state(&self, node: NodeIndex, shard: usize) -> NodeTimeState {
        let paths = self.assignments[&node][shard].clone();
        NodeTimeState {
            time: VectorTime::with_length(paths.0.iter().map(|a|a.len()).sum()),
            paths,
            base_paths: unimplemented!(),
        }
    }
}

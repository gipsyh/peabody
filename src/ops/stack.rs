use crate::node::BddPointer;
use fxhash::FxBuildHasher;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Task {
    pub left: BddPointer,
    pub right: BddPointer,
}

pub struct TaskState {
    pub stage: usize,
    pub first: Option<BddPointer>,
    pub second: Option<BddPointer>,
}

impl TaskState {
    fn from_inner(
        inner: &TaskStateInner,
        finished: &HashMap<Task, BddPointer, FxBuildHasher>,
    ) -> Self {
        Self {
            stage: inner.stage,
            first: inner.first.as_ref().map(|x| finished[x]),
            second: inner.second.as_ref().map(|x| finished[x]),
        }
    }
}

#[derive(Default)]
struct TaskStateInner {
    stage: usize,
    first: Option<Task>,
    second: Option<Task>,
}

pub struct Call {
    task: Task,
    state: TaskStateInner,
}

impl Call {
    fn new(task: Task) -> Self {
        Self {
            task,
            state: TaskStateInner::default(),
        }
    }
}

pub struct ManualStack {
    stack: Vec<Call>,
    finished: HashMap<Task, BddPointer, FxBuildHasher>,
}

impl ManualStack {
    pub fn new(task: Task, capacity: usize) -> Self {
        let mut stack = Vec::with_capacity(capacity);
        stack.push(Call::new(task));
        Self {
            stack,
            finished: HashMap::with_capacity_and_hasher(capacity, FxBuildHasher::default()),
        }
    }

    pub fn handle(&mut self) -> Option<(Task, TaskState)> {
        while let Some(call) = self.stack.last() {
            if self.finished.contains_key(&call.task) {
                self.stack.pop();
                continue;
            }
            let task = call.task;
            let state = TaskState::from_inner(&call.state, &self.finished);
            return Some((task, state));
        }
        None
    }

    pub fn pump_stage(&mut self) {
        self.stack.last_mut().unwrap().state.stage += 1;
    }

    pub fn call(&mut self, stage: usize, first: Task, second: Option<Task>) {
        let this = self.stack.last_mut().unwrap();
        this.state = TaskStateInner {
            stage,
            first: Some(first),
            second,
        };
        if let Some(task) = second {
            self.stack.push(Call::new(task))
        }
        self.stack.push(Call::new(first));
    }

    pub fn ret(&mut self, value: BddPointer) {
        let call = self.stack.pop().unwrap();
        assert!(self.finished.insert(call.task, value).is_none());
    }
}

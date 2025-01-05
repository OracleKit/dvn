use std::collections::HashMap;

use crate::task::Task;

pub struct Consensus {
    threshold: u64,
    task_approvals: HashMap<Vec<u8>, u64>
}

impl Consensus {
    pub fn new() -> Self {
        Self {
            threshold: 2,
            task_approvals: HashMap::new()
        }
    }

    pub fn consensus(&mut self, tasks: Vec<Task>) -> Vec<Task> {
        let mut approved_tasks: Vec<Task> = vec![];

        for task in tasks {
            let serialized_task = serde_json::to_vec(&task).unwrap();
            let approvals = self.task_approvals.get(&serialized_task).unwrap_or(&0);
            let mut approvals = approvals.clone();
            approvals += 1;

            self.task_approvals.insert(serialized_task, approvals);
            if approvals == self.threshold {
                approved_tasks.push(task);
            }
        }

        approved_tasks
    }
} 
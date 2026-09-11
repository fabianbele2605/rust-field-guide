#[derive(Debug, Clone)]
pub enum TaskState {
    Ready,          // Esperando ser ejecutada
    Running,        // Actualmente ejecutandose
    Blocked,        // Esperando I/O
}

#[derive(Debug, Clone)]
pub struct Context {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,       // Instruction pointer (qué linea ejecutar)
}

impl Context {
    pub fn new() -> Self {
        Context {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0,
        }
    }
}

pub struct Task {
    id: u32,
    state: TaskState,
    stack: [u8; 4096],      // 4KB stack por tarea
    pub context: Context,
}

impl Task {
    pub fn new(id: u32) -> Self {
        Task {
            id,
            state: TaskState::Ready,
            stack: [0; 4096],
            context: Context::new(),
        }
    }
}

pub struct TaskManager {
    tasks: [Option<Task>; 10],      // Máximo 10 tareas
    current_task_id: usize,         // Cual tarea esta ejecutandose
}

impl TaskManager {
    pub const fn new() -> Self {
        TaskManager {
            tasks: [const { None }; 10],
            current_task_id: 0,
        }
    }

    pub fn create_task(&mut self) -> u32 {
        for i in 0..10 {
            if self.tasks[i].is_none() {
                let task = Task::new(i as u32);
                self.tasks[i] = Some(task);
                return i as u32;
            }
        }
        panic!("No more task slots!");
    }

    pub fn current_task(&mut self) -> Option<&mut Task> {
        self.tasks[self.current_task_id].as_mut()
    }

    pub fn schedule_next(&mut self) {
        // FIFO: siguiente tarea lista (Ready)
        for i in 0..10 {
            let next_id = (self.current_task_id + 1 + i) % 10;
            if let Some(task) = &self.tasks[next_id] {
                if matches!(task.state, TaskState::Ready) {
                    self.current_task_id = next_id;
                    return;
                }
            }
        }
    }
}
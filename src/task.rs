mod task;

type TaskID u32

struct Task {
    state: TaskState,
    run: Fn(),
}

enum TaskState {
    Runnable,
    Waiting,
    Failed,
    InSend(TaskID),
    InRecv(TaskID),
}

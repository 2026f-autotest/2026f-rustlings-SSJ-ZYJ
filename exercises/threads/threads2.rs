// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.
// 在上一题的基础上，本题要求所有线程完成工作，但这次需要由线程负责更新共享值：JobStatus.jobs_completed。
//
// 执行 `rustlings hint threads2` 获取提示，或使用 watch 子命令中的 hint。

// I AM NOT DONE

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: 0 });
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // TODO：更新共享值前必须先采取相应操作。
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        // TODO：打印 JobStatus.jobs_completed 的值。注意到输出中有什么有趣的现象吗？
        // 是否必须对所有句柄调用 `join`？
        println!("jobs completed {}", ???);
    }
}

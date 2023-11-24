use std::time::Duration;

// The timer we wrote in the previous section:
use rust_async_playground::{new_executor_and_spawner, TimerFuture};

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // Spawn a task to print before and after waiting on a timer.
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        let timer1 = TimerFuture::new(Duration::new(2, 0));
        // NOTE: This async block, which evaluates in a Future,
        // makes progress only when `Poll`ed. Calling `.await` here
        // means calling `poll` on this inner future (TimerFuture).
        // The outer `poll` execution results in `Poll::Pending`
        // if any of the inner future `poll` calls returns a pending state,
        // otherwise the outer Future is ready.
        timer1.await;
        println!("done first!");
        let timer2 = TimerFuture::new(Duration::new(2, 0));
        timer2.await;

        println!("done second!");
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    // We need to do this for this reason:
    // `spawner`.spawn creates a clone to the sender, wrapped in a Task struct,
    // that is sent to the receiver of the executor. If we don't drop the spawner
    // early along with its sender ref, then when the `executor` `run`s, there always be
    // at least one ref to a sender which is in the spawner itself. As such, the thread
    // will be blocked while waiting for messages even after it has processed the task
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();
}

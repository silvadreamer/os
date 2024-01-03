// mod context;
// mod switch;
// mod task;
// mod manager;
// mod processor;
// mod pid;

// //use crate::loader::{get_num_app, get_app_data};
// //use crate::trap::TrapContext;
// //use crate::sync::UPSafeCell;
// use lazy_static::*;
// use switch::__switch;
// use task::{TaskControlBlock, TaskStatus};
// use alloc::vec::Vec;
// use crate::loader::get_app_data_by_name;
// use manager::add_task;

// pub use context::TaskContext;
// // pub use processor::{
// //     run_tasks,
// //     current_task,
// //     current_user_token,
// //     current_trap_cx,
// //     take_current_task,
// //     schedule,
// // };
// pub use pid::{PidHandle, pid_alloc, KernelStack};
// // pub struct TaskManager {
// //     num_app: usize,
// //     inner: UPSafeCell<TaskManagerInner>,
// // }

// // struct TaskManagerInner {
// //     tasks: Vec<TaskControlBlock>,
// //     current_task: usize,
// // }

// // lazy_static! {
// //     pub static ref TASK_MANAGER: TaskManager = {
// //         println!("init TASK_MANAGER");
// //         let num_app = get_num_app();
// //         println!("num_app = {}", num_app);
// //         let mut tasks: Vec<TaskControlBlock> = Vec::new();
// //         for i in 0..num_app {
// //             tasks.push(TaskControlBlock::new(
// //                 get_app_data(i),
// //                 i,
// //             ));
// //         }
// //         TaskManager {
// //             num_app,
// //             inner: unsafe { UPSafeCell::new(TaskManagerInner {
// //                 tasks,
// //                 current_task: 0,
// //             })},
// //         }
// //     };
// // }

// impl TaskManager {
//     fn run_first_task(&self) -> ! {
//         let mut inner = self.inner.exclusive_access();
//         let next_task = &mut inner.tasks[0];
//         next_task.task_status = TaskStatus::Running;
//         let next_task_cx_ptr = &next_task.task_cx as *const TaskContext;
//         drop(inner);
//         let mut _unused = TaskContext::zero_init();
//         // before this, we should drop local variables that must be dropped manually
//         unsafe {
//             __switch(
//                 &mut _unused as *mut _,
//                 next_task_cx_ptr,
//             );
//         }
//         panic!("unreachable in run_first_task!");
//     }

//     fn mark_current_suspended(&self) {
//         let mut inner = self.inner.exclusive_access();
//         let cur = inner.current_task;
//         inner.tasks[cur].task_status = TaskStatus::Ready;
//     }

//     fn mark_current_exited(&self) {
//         let mut inner = self.inner.exclusive_access();
//         let cur = inner.current_task;
//         inner.tasks[cur].task_status = TaskStatus::Exited;
//     }

//     fn find_next_task(&self) -> Option<usize> {
//         let inner = self.inner.exclusive_access();
//         let current = inner.current_task;
//         (current + 1..current + self.num_app + 1)
//             .map(|id| id % self.num_app)
//             .find(|id| {
//                 inner.tasks[*id].task_status == TaskStatus::Ready
//             })
//     }

//     fn get_current_token(&self) -> usize {
//         let inner = self.inner.exclusive_access();
//         inner.tasks[inner.current_task].get_user_token()
//     }

//     fn get_current_trap_cx(&self) -> &mut TrapContext {
//         let inner = self.inner.exclusive_access();
//         inner.tasks[inner.current_task].get_trap_cx()
//     }

//     fn run_next_task(&self) {
//         if let Some(next) = self.find_next_task() {
//             let mut inner = self.inner.exclusive_access();
//             let current = inner.current_task;
//             inner.tasks[next].task_status = TaskStatus::Running;
//             inner.current_task = next;
//             let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
//             let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
//             drop(inner);
//             // before this, we should drop local variables that must be dropped manually
//             unsafe {
//                 __switch(
//                     current_task_cx_ptr,
//                     next_task_cx_ptr,
//                 );
//             }
//             // go back to user mode
//         } else {
//             panic!("All applications completed!");
//         }
//     }
// }

// pub fn run_first_task() {
//     TASK_MANAGER.run_first_task();
// }

// fn run_next_task() {
//     TASK_MANAGER.run_next_task();
// }

// fn mark_current_suspended() {
//     TASK_MANAGER.mark_current_suspended();
// }

// fn mark_current_exited() {
//     TASK_MANAGER.mark_current_exited();
// }

// pub fn suspend_current_and_run_next() {
//     // There must be an application running.
//     let task = take_current_task().unwrap();

//     // ---- access current TCB exclusively
//     let mut task_inner = task.inner_exclusive_access();
//     let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
//     // Change status to Ready
//     task_inner.task_status = TaskStatus::Ready;
//     drop(task_inner);
//     // ---- release current PCB

//     // push back to ready queue.
//     add_task(task);
//     // jump to scheduling cycle
//     schedule(task_cx_ptr);
// }

// // pub fn exit_current_and_run_next() {
// //     mark_current_exited();
// //     run_next_task();
// // }

// pub fn exit_current_and_run_next(exit_code: i32) {
//     // take from Processor
//     let task = take_current_task().unwrap();
//     // **** access current TCB exclusively
//     let mut inner = task.inner_exclusive_access();
//     // Change status to Zombie
//     inner.task_status = TaskStatus::Zombie;
//     // Record exit code
//     inner.exit_code = exit_code;
//     // do not move to its parent but under initproc

//     // ++++++ access initproc TCB exclusively
//     {
//         let mut initproc_inner = INITPROC.inner_exclusive_access();
//         for child in inner.children.iter() {
//             child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
//             initproc_inner.children.push(child.clone());
//         }
//     }
//     // ++++++ release parent PCB

//     inner.children.clear();
//     // deallocate user space
//     inner.memory_set.recycle_data_pages();
//     drop(inner);
//     // **** release current PCB
//     // drop task manually to maintain rc correctly
//     drop(task);
//     // we do not have to save task context
//     let mut _unused = TaskContext::zero_init();
//     schedule(&mut _unused as *mut _);
// }

// pub fn current_user_token() -> usize {
//     TASK_MANAGER.get_current_token()
// }

// pub fn current_trap_cx() -> &'static mut TrapContext {
//     TASK_MANAGER.get_current_trap_cx()
// }

// lazy_static! {
//     pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(
//         TaskControlBlock::new(get_app_data_by_name("initproc").unwrap())
//     );
// }

// pub fn add_initproc() {
//     add_task(INITPROC.clone());
// }

mod context;
mod switch;
mod task;
mod manager;
mod processor;
mod pid;

use crate::loader::get_app_data_by_name;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};
use alloc::sync::Arc;
use manager::fetch_task;

pub use context::TaskContext;
pub use processor::{
    run_tasks,
    current_task,
    current_user_token,
    current_trap_cx,
    take_current_task,
    schedule,
};
pub use manager::add_task;
pub use pid::{PidHandle, pid_alloc, KernelStack};

pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current PCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

pub fn exit_current_and_run_next(exit_code: i32) {
    // take from Processor
    let task = take_current_task().unwrap();
    // **** access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    // Change status to Zombie
    inner.task_status = TaskStatus::Zombie;
    // Record exit code
    inner.exit_code = exit_code;
    // do not move to its parent but under initproc

    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    // ++++++ release parent PCB

    inner.children.clear();
    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // **** release current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
}

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(
        TaskControlBlock::new(get_app_data_by_name("initproc").unwrap())
    );
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

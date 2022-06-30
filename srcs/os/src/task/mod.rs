//! Task management implementation
//!
//! Everything about task management, like starting and switching tasks is
//! implemented here.
//!
//! A single global instance of [`TaskManager`] called `TASK_MANAGER` controls
//! all the tasks in the whole operating system.
//!
//! A single global instance of [`Processor`] called `PROCESSOR` monitors running
//! task(s) for each core.
//!
//! A single global instance of [`PidAllocator`] called `PID_ALLOCATOR` allocates
//! pid for user apps.
//!
//! A single global instance of [`Vec<IpcMessage>`] called `IPC_CHANNEL` allocated ipc message
//!
//! A single global instance of [`Vec<SysLock>`] call `GOLBAL_LOCK` allows lock usage
//!
//! Be careful when you see `__switch` ASM function in `switch.S`. Control flow around this function
//! might not be what you expect.
mod context;
mod manager;
mod pid;
mod processor;
mod switch;
#[allow(clippy::module_inception)]
#[allow(rustdoc::private_intra_doc_links)]
mod task;

use crate::{
    console::print,
    fs::{open_file, OpenFlags},
    mm::{translated_byte_buffer, translated_refmut, translated_str, UserBuffer},
    sync::UPSafeCell,
};
use alloc::{sync::Arc, vec::Vec};
pub use context::TaskContext;
use lazy_static::*;
pub use manager::{fetch_task, TaskManager};
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use manager::add_task;
pub use pid::{pid_alloc, KernelStack, PidAllocator, PidHandle};
pub use processor::{
    current_task, current_trap_cx, current_user_token, run_tasks, schedule, take_current_task,
    Processor,
};

use self::manager::TASK_MANAGER;
/// Suspend the current 'Running' task and run the next task in task list.
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
/// Exit the current 'Running' task and run the next task in task list.
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
    ///Globle process that init user shell
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new({
        let inode = open_file("initproc", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskControlBlock::new(v.as_slice())
    });
}

///Add init process to the manager
pub fn add_initproc() {
    add_task(INITPROC.clone());
}

///Basic IPC message struct
pub struct IpcMessage {
    from_pid: usize,
    to_pid: usize,
    message: usize,
    size: usize,
}

impl IpcMessage {
    /// Construct an IPC message
    pub fn new(from_pid: usize, to_pid: usize, message: usize, size: usize) -> Self {
        IpcMessage {
            from_pid,
            to_pid,
            message,
            size,
        }
    }

    /// translate pid into from_pid and to_pid
    pub fn translate_pid(pid: usize) -> [usize; 2] {
        let from_pid = (pid & 0xffffffff00000000) >> 32;
        let to_pid = pid & 0x00000000ffffffff;
        [from_pid, to_pid]
    }
}

///A recv request
pub struct IpcRequest {
    pid: usize,
    buffer: usize,
    size: usize,
}

impl IpcRequest {
    /// Construct an IPC request
    pub fn new(pid: usize, buffer: usize, size: usize) -> Self {
        IpcRequest { pid, buffer, size }
    }
}

lazy_static! {
    ///Init IPC
    pub static ref IPC_CHANNEL: UPSafeCell<Vec<IpcMessage>> = unsafe {UPSafeCell::new(Vec::new())};
}

///Add message to the channel
pub fn add_message(message: IpcMessage) {
    IPC_CHANNEL.exclusive_access().push(message)
}

///Try to receive message from the channel
pub fn receive_message(mut request: IpcRequest) -> isize {
    let channel = IPC_CHANNEL.exclusive_access();
    for message in channel.iter() {
        if message.to_pid == request.pid {
            // read from origin sender
            if let Some(task) = TASK_MANAGER.exclusive_access().get(message.from_pid) {
                let token = task.inner_exclusive_access().get_user_token();
                let message_buffer = UserBuffer::new(translated_byte_buffer(
                    token,
                    message.message as *const u8,
                    message.size,
                ));
                // write to the receiver buffer
                let curr_token = current_user_token();
                let write_buffer = UserBuffer::new(translated_byte_buffer(
                    curr_token,
                    request.buffer as *const u8,
                    request.size,
                ));
                for (write_char, read_char) in write_buffer.into_iter().zip(message_buffer) {
                    request.size -= 1;
                    request.buffer += 1;
                    unsafe { *write_char = *read_char };
                }
            }
        }
    }
    0
}

///Serve as a lock for user
pub struct Lock(pub usize);

impl Lock {
    ///get inner value
    pub fn get(&self) -> usize {
        return self.0;
    }
    ///set inner value
    pub fn set(&mut self, val: usize) {
        self.0 = val
    }
}

///Lock struct
pub struct SysLock {
    lock: Lock,
    id: usize,
}

lazy_static! {
    ///Init Lock
    pub static ref GLOBAL_LOCK: UPSafeCell<Vec<SysLock>> = unsafe {UPSafeCell::new(Vec::new())};
    ///lock count
    pub static ref LOCK_COUNT: UPSafeCell<usize> = unsafe {UPSafeCell::new(0)};
}

///get lock count
pub fn lock_count() -> usize {
    LOCK_COUNT.exclusive_access().clone()
}

///return id of the lock allocated
pub fn lock_acquire() -> usize {
    let id = lock_count();
    let lock = SysLock { lock: Lock(0), id };
    GLOBAL_LOCK.exclusive_access().push(lock);
    id
}

///return lock contained value, 0 is default
pub fn lock_get(id: usize) -> usize {
    let locks = GLOBAL_LOCK.exclusive_access();
    for lock in locks.iter() {
        if lock.id == id {
            return lock.lock.get();
        }
    }
    0
}

///set lock contained value
pub fn lock_set(id: usize, val: usize) {
    let mut locks = GLOBAL_LOCK.exclusive_access();
    for lock in locks.iter_mut() {
        if lock.id == id {
            lock.lock.set(val);
            break;
        }
    }
}

///release lock
pub fn lock_release(id: usize) {
    let mut locks = GLOBAL_LOCK.exclusive_access();
    locks.retain(|x| x.id != id);
}

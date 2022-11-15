//! Process management syscalls

use crate::config::{MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, get_current_task_info};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    // 任务控制块相关信息（任务状态）
    pub status: TaskStatus,
    // 任务使用的系统调用及调用次数
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    // 任务总运行时长（单位ms）
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// 参数：ti: 待查询任务信息
/// 返回值：执行成功返回0，错误返回-1
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let ti = unsafe { &mut *ti };
    get_current_task_info(ti)
}
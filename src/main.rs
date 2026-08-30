//! BORUIX `synce2e`：SYNC 域（ADR-032）端到端阻塞往返测试的**等待端**。
//!
//! 由协调进程（shell 的 `synce2e` 内建命令）经 `exec_path("/programs/synce2e.elf",
//! "waiter:<id>")` 派生为子进程。本程序读取 argv[0] 中的同步字 id，然后调用
//! `sync_wait(id, 0, 0)` **真正阻塞**（在内核 SYNC_TABLE 登记为等待者、进程切换
//! 挂起），直到协调端 `sync_wake(id, 42, 1)` 把它唤醒并把新值 42 预置进 saved.rax；
//! 恢复后 `sync_wait` 返回 42。若返回的不是 42 则以非零退出码如实失败。
//!
//! 退出码约定：0 = 拿到唤醒值 42（往返成功）；非零 = 失败。协调端用
//! `waitpid_any()` 收该退出码断言。

#![no_std]
#![no_main]

use libsys::{sync_wait, write, STDOUT};

/// 把无符号整数格式化为十进制字符串（写入缓冲），返回有效切片。
fn u64_to_dec(v: u64, buf: &mut [u8; 24]) -> &[u8] {
    let mut i = buf.len();
    let mut n = v;
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 { break; }
    }
    &buf[i..]
}

/// 输出一行到 stdout。
fn say(s: &[u8]) {
    let _ = write(STDOUT, s);
    let _ = write(STDOUT, b"\n");
}

/// 解析 argv[0] = `waiter:<十进制 id>`，返回 id。
fn parse_waiter_id(arg: &[u8]) -> Result<u64, ()> {
    let prefix = b"waiter:";
    let idb = arg.strip_prefix(prefix).ok_or(())?;
    if idb.is_empty() { return Err(()); }
    let mut id: u64 = 0;
    for &c in idb {
        if !c.is_ascii_digit() { return Err(()); }
        id = id.checked_mul(10)
            .and_then(|v| v.checked_add((c - b'0') as u64))
            .ok_or(())?;
    }
    Ok(id)
}

/// 用户程序入口（libsys `_start` 调用）。返回值为进程退出码。
#[unsafe(no_mangle)]
pub extern "C" fn user_main(argc: isize, argv: *const *const u8) -> i32 {
    let cmd: &[u8] = unsafe {
        if argc < 1 || argv.is_null() { &[] } else {
            let p = *argv;
            if p.is_null() { &[] } else {
                let mut len = 0usize;
                while *p.add(len) != 0 { len += 1; }
                core::slice::from_raw_parts(p, len)
            }
        }
    };

    let id = match parse_waiter_id(cmd) {
        Ok(id) => id,
        Err(()) => {
            say(b"synce2e[waiter]: bad cmd (expected waiter:<id>)");
            return 2;
        }
    };

    let mut b = [0u8; 24];
    say(b"synce2e[waiter]: blocking on sync_wait");
    let val = match sync_wait(id, 0, 0) {
        Ok(v) => v,
        Err(_) => {
            say(b"synce2e[waiter]: sync_wait returned error");
            return 3;
        }
    };

    if val == 42 {
        say(b"synce2e[waiter]: woke with value 42 OK");
        0
    } else {
        say(b"synce2e[waiter]: woke with WRONG value (expected 42):");
        let _ = write(STDOUT, u64_to_dec(val, &mut b));
        let _ = write(STDOUT, b"\n");
        4
    }
}

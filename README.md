# synce2e

**简体中文** | [English](#english)

BORUIX 的**跨进程同步端到端测试**——验证"一个进程等待，另一个进程唤醒它"这条完整链路。

它是这个测试的**等待端**：启动后向指定的同步字登记为等待者并**真正阻塞**，直到另一个进程把它
唤醒，然后检查收到的值是否正确。

```
synce2e[waiter]: blocking on sync_wait
synce2e[waiter]: woke with value 42 OK
```

---

## 它验证什么

跨进程同步的完整流程有三个环节，缺一不可：

| 环节 | 说明 |
| --- | --- |
| **登记** | 等待方告诉内核"我在等这个同步字"，然后挂起 |
| **唤醒** | 另一个进程发出唤醒，内核把等待方标记为可运行 |
| **取值** | 等待方恢复运行，并**拿到唤醒方传递的值** |

这个测试要证明的是**三个环节全部真实发生**，而不是只有表面行为。

关键在于**"真正阻塞"**这一点：如果等待方只是空转轮询，那么它能"等到"值并不说明内核的阻塞
与唤醒机制工作正常——它只是碰巧在轮询中读到了。所以测试要求等待方**确实挂起**，让出 CPU，
被调度器移出运行队列，然后由唤醒动作把它拉回来。

**值的传递同样重要**：唤醒不只是"叫醒"，还要携带一个值。等待方恢复后必须拿到这个值。这验证
的是恢复时的**寄存器状态**被正确处理——唤醒方给的值在沉睡期间不能丢失。

## 它怎么被使用

这个程序**不能单独运行出完整结果**——它只扮演等待端，另一半（唤醒端）在 shell 里。

正常用法是通过系统内建的同步测试命令启动：协调方先创建同步字、启动本程序作为子进程、等它进入
等待状态，然后发出唤醒并携带一个约定的值。最后协调方检查子进程的退出码来判定结果。

这样设计的目的是让**两个进程真正在不同的执行流里**，而不是在同一个程序内模拟等待与唤醒。

## 两个进程如何约定

启动时通过命令行参数接收要等待的同步字编号：

```
waiter:<编号>
```

参数格式不符时程序如实报错并退出。

> **注意**：系统的参数约定是**参数个数恒为 1，整条命令行放在第一个参数里**。所以这里读的是
> 第一个参数，而不是按传统习惯读第二个位置。这是本系统上最容易踩的一处约定。

## 退出码

| 退出码 | 含义 |
| --- | --- |
| `0` | 拿到正确的唤醒值，往返成功 |
| `2` | 命令行参数格式错误 |
| `3` | 等待调用本身返回错误 |
| `4` | 被唤醒了，但**收到的值不对** |

区分 `3` 和 `4` 是有意的：

- **`3`** 说明等待这条路根本没走通——登记或挂起阶段就出了问题
- **`4`** 说明**阻塞和唤醒都成功了**，但值在传递过程中出了问题

两者的排查方向完全不同。如果只返回一个笼统的失败码，就要靠翻日志才能分辨。

协调方通过等待子进程并读取退出码来断言结果，因此**退出码本身就是测试结论**。

## 一个可能让人困惑的地方：它为什么"卡住了"

这个程序启动后**看起来像是卡住了**——它打印一行"正在阻塞"之后就再也没有输出，直到被唤醒。

**这是预期行为，不是故障**。它的全部意义就在于真实地阻塞。如果没有别的进程来唤醒它，它会
一直等下去。

## 构建

```bash
cargo build --release
```

编译产物部署为 BORUIX 系统中的用户态程序，由协调方拉起。

## 文件结构

```
synce2e/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 等待端本体
```

## 相关项目

- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 提供同步字的等待与唤醒接口
- [`shell`](https://github.com/BRX-Boruix/shell) —— 提供协调端命令
- [`selftest`](https://github.com/BRX-Boruix/selftest) —— 系统自检

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

---

# English

[简体中文](#synce2e) | **English**

An **end-to-end test of cross-process synchronisation** for BORUIX — it verifies the whole chain
where one process waits and another wakes it.

It is the **waiting side**: after starting, it registers itself as a waiter on the given
synchronisation word and **genuinely blocks**, until another process wakes it, whereupon it checks
that the value it received is correct.

```
synce2e[waiter]: blocking on sync_wait
synce2e[waiter]: woke with value 42 OK
```

---

## What it verifies

The full cross-process synchronisation flow has three stages, none dispensable:

| Stage | Meaning |
| --- | --- |
| **Registration** | The waiter tells the kernel "I am waiting on this word", then suspends |
| **Waking** | Another process signals a wake, and the kernel marks the waiter runnable |
| **Value transfer** | The waiter resumes and **receives the value the waker passed** |

The test must show that **all three really happen**, not merely that the surface behaviour looks
right.

The crux is that "**genuinely blocks**". If the waiter merely spun in a poll loop, then "receiving"
the value would prove nothing about the kernel's blocking and waking machinery — it would just have
happened to observe the value while polling. So the test requires the waiter to **actually suspend**,
yield the CPU, be removed from the run queue, and then be pulled back by the wake.

**The value transfer matters just as much**: waking is not merely "rousing" — it carries a value. On
resuming, the waiter must receive it. That verifies that the **register state** is handled correctly
on resume — the value the waker supplied must not be lost while the process sleeps.

## How it is used

This program **cannot produce a complete result on its own** — it is only the waiting side; the other
half (the waking side) lives in the shell.

The normal route is the system's builtin synchronisation test command: the coordinator creates the
synchronisation word, starts this program as a child, waits for it to enter the waiting state, then
signals a wake carrying an agreed value. Finally the coordinator inspects the child's exit code to
decide the outcome.

The point of the arrangement is to keep the **two processes genuinely in different execution
flows**, rather than simulating waiting and waking inside one program.

## How the two sides agree

At startup it receives the number of the synchronisation word to wait on, as a command line argument:

```
waiter:<number>
```

A malformed argument is reported honestly and the program exits.

> **Note**: the system's argument convention is that **the argument count is always 1, with the whole
> command line in the first argument**. So this reads the first argument rather than reaching for the
> second position out of habit. It is the easiest convention to trip over on this system.

## Exit codes

| Exit code | Meaning |
| --- | --- |
| `0` | Received the correct wake value — the round trip succeeded |
| `2` | The command line argument was malformed |
| `3` | The wait call itself returned an error |
| `4` | It was woken, but the **value received was wrong** |

Distinguishing `3` from `4` is deliberate:

- **`3`** means the wait path never worked — the failure is at registration or suspension
- **`4`** means **blocking and waking both succeeded**, but the value was mishandled in transfer

The two point at entirely different investigations. A single blanket failure code would force a dig
through logs to tell them apart.

The coordinator asserts by waiting on the child and reading its exit code, so **the exit code is
itself the test verdict**.

## One thing that may look confusing: why it appears stuck

After starting, this program **looks stuck** — it prints one "blocking" line and then nothing until it
is woken.

**That is the expected behaviour, not a fault.** Genuinely blocking is its entire purpose. Without
another process to wake it, it waits indefinitely.

## Building

```bash
cargo build --release
```

The artifact is deployed as a user-space program in a BORUIX system, started by the coordinator.

## Layout

```
synce2e/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the waiting side itself
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — provides the synchronisation word's wait and wake interfaces
- [`shell`](https://github.com/BRX-Boruix/shell) — provides the coordinating command
- [`selftest`](https://github.com/BRX-Boruix/selftest) — the system self-test

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).

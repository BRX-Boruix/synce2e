# synce2e

BORUIX 的**端到端验收程序**，验证同步字（sync word）的阻塞等待与唤醒往返。

[English](README.en.md)

## 测什么

这是往返测试的**等待端**：它调用同步等待接口**真正阻塞**（在内核登记为等待者、进程切换挂起），
直到协调端把它唤醒并传入一个值。程序断言唤醒值正是期望的 `42`，否则如实失败。

它验证的关键路径是**真实的进程上下文切换**——阻塞与唤醒要求进程真的被挂起再恢复，这是内核态
自检无法覆盖的。

## 用法

由 shell 的内建命令 `synce2e` 派生为子进程，参数为 `waiter:<id>`，其中 `<id>` 是同步字编号。

```
synce2e[waiter]: blocking on sync_wait
synce2e[waiter]: woke with value 42 OK
```

协调端（shell 侧）负责执行唤醒，并用 `waitpid` 收取本进程的退出码作为裁决。

## 退出码

| 退出码 | 含义 |
| --- | --- |
| `0` | 拿到唤醒值 42，往返成功 |
| `2` | 参数格式错误（非 `waiter:<id>`） |
| `3` | 等待调用本身返回错误 |
| `4` | 被唤醒，但拿到的值不是 42 |

## 构建

```bash
cargo build --release
```

编译产物部署为 `/programs/synce2e.elf`。

## 文件结构

```
synce2e/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 等待端逻辑与判定
```

## 相关项目

- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 提供同步等待接口
- [`shell`](https://github.com/BRX-Boruix/shell) —— 协调端，负责派生与唤醒

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

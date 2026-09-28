# synce2e

BORUIX 的端到端验收程序，验证同步字的阻塞等待与唤醒往返。

[English](README.en.md)

## 测什么

这是往返测试的等待端：调用同步等待接口**真正阻塞**——进程被挂起、切换到别的进程——直到协调端
把它唤醒并传入一个值。程序断言唤醒值是 `42`。

它覆盖的是真实的进程上下文切换：阻塞与唤醒要求进程真的被挂起再恢复，这是内核态自检覆盖不到的
路径。

## 用法

不单独运行。由 shell 的内建命令 `synce2e` 派生为子进程，命令行为 `waiter:<id>`：

```
synce2e[waiter]: blocking on sync_wait
synce2e[waiter]: woke with value 42 OK
```

协调端负责创建同步字、派生本程序、执行唤醒，并收取本进程的退出码裁决。

## 退出码

- `0`——拿到唤醒值 42，往返成功
- `2`——命令行格式错误（不是 `waiter:<数字>`）
- `3`——等待调用本身返回错误
- `4`——被唤醒，但值不是 42

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
    └── main.rs   # 参数解析、阻塞等待与判定
```

## 相关项目

- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 同步等待接口
- [`shell`](https://github.com/BRX-Boruix/shell) —— 协调端：创建、派生、唤醒、收退出码

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

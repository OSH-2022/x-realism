# YAOG 调研报告

部分选题参考该仓库: https://github.com/oscomp.

## Rust 重写

- zCore https://github.com/oscomp/proj9-zcore (叶升宇提出)

    在现有 zCore 的基础上, 进一步完成如下目标:

    + 分阶段重新实现简化版 zCore 

    + 提供 Linux 内核的系统服务

    + 提供 Zircon 内核的系统服务

    + 完善多核支持及异步调度机制

    > 老师排除, 因为架构上并不占优.

- 轻量级全容器化 OS https://github.com/oscomp/proj150-Rust-ContainerOS (刘良宇提出)

    用 Rust 语言实现一个轻量级的全容器化 OS, 可参考 RancherOS 的实现.
    
    > 相关资料不够完善, 对该领域了解也不够深入, 排除.
    
- **用 Rust 语言从零开始写操作系统内核**, 可以参考 https://rcore-os.github.io/rCore-Tutorial-Book-v3/ (叶升宇提出)

  > **最终被采纳**. 因为小组成员认为通过该方向可以加深对操作系统的理解, 培养操作系统架构设计能力, 同时可探索方向也比较丰富.

## 文件系统

+ 改写 DisGraFS, 使其拥有更高的性能. (黄瑞轩提出)

    > 与小组成员兴趣不符, 排除.

+ https://github.com/oscomp/proj46-AliOS-Things-CloudFS (许坤钊提出)
    
    + 将 OSS 功能在 AliOS Things 上面运行起来, 能本地文件上传、下载云端文件、获取云端文件信息
    + 将 OSS 功能对接到 AliOS Things VFS 模块
    + 云文件系统的目录缓存, 云文件本地缓存, 端云状态同步, 断点恢复等功能.
    
    > 老师排除, 因为从设计以及未来发展趋势的角度上看, 不值得.

## 虚拟化

+ Unikernel, 选一个语言实现. (许坤钊提出)

    > 先前的 OSH 大作业中已经有许多小组进行了探索与实现, 为了避免同质化, 排除.

+ 在 RISC-V 处理器上实现一个轻量级的 Hypervisorhttps://github.com/oscomp/proj23-lightweight-hypervisor (黄瑞轩提出)

    > 经小组成员讨论认为可行性不强, 故排除.

## Raspberry

+ 造轮子, 参考 [x-ridiculous-includeos](https://github.com/OSH-2019/x-ridiculous-includeos), 移植一个操作系统到树莓派上. (刘良宇提出)

    > 小组成员认为可探索方向有限, 排除.


# x-realism 结题报告

## 1 项目简介

内核是操作系统的核心，它是硬件和计算机进程之间的**主要接口**。内核将这两者连接起来，以便尽可能有效地调度资源。本项目旨在基于现有的轮子，吸纳多个平台的优点，实现一个我们自己的操作系统内核。

我们的操作系统内核计划是**微内核架构**的。微内核是提供操作系统核心功能的内核的精简版本，它能在很小的内存空间内增加移植性，提供模块化设计，以使用户安装不同的接口。

从已有的微内核操作系统经验来看，性能问题（主要涉及进程间通信，上下文切换的高开销）一直备受关注，我们期望就此部分进行优化，提高效率。

我们的建构思路是从对应用由简到繁的支持角度出发，满足应用的阶段性需求。根据特性（需求）逐步添加或增强操作系统功能，最终形成一个简单但相对完善的操作系统内核。我们期望通过此项目来加强对操作系统相关理论概念的理解，同时掌握操作系统设计的能力。

## 2 背景和立项依据

### 2.1 项目背景

#### 2.1.1 目前流行的 OS 的不足

- Windows（混合内核）
  - 系统稳定性差。Windows 的各个程序不是相互独立的，软件的崩溃容易导致系统瘫痪。
  - 软件管理安装机制差，软件和软件之间不隔离配置，而是共用一个庞大的全局注册表，各个软件有自己设计的安装和卸载机制，从而很难说删除“干净”某个软件。
  - 安全性差，即使存在自带的 Windows Defender，电脑仍然容易遭受病毒的攻击，因此常常需要不断地进行系统更新以获取最新的安全补丁。

- Mac OS（混合内核）
  - 硬件升级和定制化困难，因为 Mac 电脑的 CPU 和内存等与其他硬件和操作系统的耦合性很高，更换硬件可能导致系统拒绝启动。

- Linux（宏内核）
  - 驱动问题。Linux 无法做到系统与驱动分离，驱动没有稳定的接口，内核变动时驱动就得跟着变动，导致驱动的开发困难，很多设备缺乏好用的驱动。
  - Linux 内核是宏内核，可移植性较差，同时很多系统服务进程运行在内核态，服务的故障会影响整个系统。

#### 2.1.2 微内核原始架构设计

微内核与宏内核设计的比较可用一张表来概述：

|          | 宏内核 | 微内核 |
| :------: | :----: | :----: |
|  模块化  |        |   √    |
|  复杂性  |   √    |        |
|  灵活性  |        |   √    |
| 可维护性 |        |   √    |
|   安全   |        |   √    |
|   性能   |   √    |        |
|  兼容性  |        |   √    |

微内核是可以提供实现操作系统所需机制的近乎最少数量的软件。这些机制包括低级地址空间管理、线程管理和进程间通信。其基本架构如下面右图所示。

![img](https://upload.wikimedia.org/wikipedia/commons/thumb/6/67/OS-structure.svg/2880px-OS-structure.svg.png)

微内核必须提供一些核心功能。至少包括：

- 处理地址空间的机制，更具体地说是内存管理机制
- 一些用于管理 CPU 分配的执行抽象，通常是**进程**
- **进程间通信**，需要调用在它们自己的地址空间中运行的 Server

其他一切都可以在用户模式程序中完成（尽管在某些处理器架构上实现为用户程序的设备驱动程序可能需要特殊权限才能访问 I/O 硬件）。

微内核的一个关键组件是**良好的 IPC 系统**和**虚拟内存管理器设计**，它允许以安全的方式在用户模式服务器中实现页面错误处理和交换。由于所有服务都由用户模式程序执行，因此程序之间的有效通信方式是必不可少的，远比单片内核更重要。IPC 系统不仅必须具有低开销，而且还必须与 CPU 调度很好地交互。

#### 2.1.3 目前流行的微内核和单内核 OS 设计的不足

- 早期，微内核操作系统利用硬件隔离机制来实现故障隔离和恢复。Minix 3 是当时著名的微内核操作系统。然而，传统的操作系统设计无法满足日益增长的可靠性和安全性要求。
- Redox 是 Rust 中的微内核操作系统，但它缺乏故障恢复机制。
- Theseus 是一个优秀的单内核操作系统，具有故障恢复功能。然而，作为单内核操作系统，它无法将用户程序从内核模块中分离出来，这使得恶意用户任务危及整个系统。

![image-20220702210601793](pic/image-20220702210601793.png)

### 2.2 立项依据

#### 2.2.1 rCore 项目

rCore 项目旨在一步一步展示如何从零开始用 Rust 语言写一个基于 RISC-V 架构的类 Unix 内核 。其目标是以简洁的 RISC-V 基本架构为底层硬件基础，根据上层应用从小到大的需求，按 OS 发展的历史脉络，逐步讲解如何设计实现能满足“从简单到复杂”应用需求的多个“小”操作系统。并且在设计实现操作系统的过程中，逐步解析操作系统各种概念与原理的知识点，做到有“理”可循和有“码”可查，最终让学生通过操作系统设计与实现来深入地掌握操作系统的概念与原理。

以上这些想法让 rCore 项目具有与我们项目匹配的天生优势——简单、清楚。即使 rCore 其实是一种典型的宏内核设计，我们也很容易在其上做出改动，让其符合我们的微内核设计。

![image-20220702231913601](pic/image-20220702231913601.png)

rCore 已经完成的工作有：进程概念管理、段页式内存管理、基于文件概念的 IPC、文件系统。本项目将以 rCore 实现到文件系统的代码为基本框架，在此基础上实现内核的 Basic IPC，并逐步删去其内核对文件系统的依赖，最终实现微内核架构。

#### 2.2.2 L4

L4 是一种微内核构架的操作系统内核，最初由约亨·李德克（Jochen Liedtke）设计，前身为 L3 微内核。后序发展上，L4 主要用于类 Unix、可移植操作系统接口（POSIX）兼容类型。

L4 秉承极简，高效和安全的设计理念：

- Mach 的 IPC 运行缓慢的一个很重要原因是 IPC 代码段过大，会发生较多 L1 cache miss，很影响时间。这启发了微内核的一个设计逻辑： **微内核本身必须充分小**。为此，L4 以及它的前身 L3 的很多代码采用汇编语言编写。
- L4 使用同步 IPC，这意味着一个集合通信模型，当发送者和接收者都准备好时交换消息。如果两者都在同一个内核上运行，这意味着其中一个将阻塞，直到另一个调用 IPC 操作。
- 在 L4 中，IPC 是通过 Endpoint 来实现的。Endpoint 可以被认为是一个邮箱，发送者和接收者通过该邮箱通过握手交换消息。任何拥有 Send 能力的人都可以通 Endpoint 发送消息，任何拥有 Receive 权限的人都可以接收消息。这意味着每个端点可以有任意数量的发送者和接收者。特别是，无论有多少线程尝试从 Endpoint 接收，特定消息仅传递给一个接收者（队列中的第一个接收者）。

#### 2.2.3 seL4

seL4 是 L4 微内核家族的一员，其着重强化了 L4 内核的安全性。seL4 的形式验证为在系统中运行的应用程序之间提供了最高的 *隔离* 保证，这意味着可以控制系统某个部分的妥协并防止损害系统的其他可能更关键的部分。

具体来说，seL4 的实现在形式上通过不同层次的接口的抽象以及每一层次的状态机形式验证被证明是正确的，并且如果配置正确，它的操作也已被证明在最坏情况下执行时间具有安全上限。

**地址空间**：根任务可以实施其资源管理策略，例如通过将系统划分为安全域并将每个域交给一个不相交的无类型内存子集。用户空间可直接访问的唯一对象是“框架对象”：这些对象可以被映射到页表，之后用户空间可以写入由这些框架对象表示的物理内存。简而言之，seL4 将内核资源的管理导出到用户级别，并使它们受到与用户资源相同的基于能力的访问控制。

**通信**：通信可以通过 IPC 或共享内存进行。IPC 通常应用于短消息，不长于几百字节的消息大小，这是依赖实现定义和体系结构的限制，但通常消息应该保持在几十个字节。对于较长的消息，应使用共享缓冲区。共享缓冲区访问可以通过通知机制同步。IPC有两种支持形式：通过端点传递的同步消息（类似端口的目的地，没有内核内缓冲），以及通过异步端点传递的异步通知（由单个内核内字组成的集合对象，用于使用逻辑or组合IPC发送）。远程过程调用语义通过应答功能在同步IPC上实现。发送功能由初始端点功能生成。

**快速路径 IPC**：在任何一个系统调用上，内核入口机制都直接调用快速路径代码。快速路径的前半部分检查当前情况是否属于优化情况。如果是这样，则快速路径的后半部分处理系统调用。如果不是，则快速路径将回调标准seL4系统调用入口点（有时称为慢路径），该入口点处理更一般的情况。此控制流如图所示。

<img src="./pic/image-20220628202736172.png" alt="image-20220628202736172" style="zoom:50%;" />

#### 2.2.4 Rust for OS

最初的 Unix 系统是完全用汇编语言写出来的，之后 B 语言和 NB (New B) 语言都被使用过。由于这些语言中只能处理计算机字节，没有类型并且不支持浮点运算，Dennis Ritchie 发明了 C 语言，C 语言从那以后就成为了开发操作系统最流行的编程语言。如今主流操作系统内核的少数部分也用 C++ 实现。

但是编写操作系统内核并不是只能用汇编跟 C，C++，一门语言能否用于编写操作系统，取决于其二进制代码是否能够在裸机上执行（也即不依赖标准库），因为标准库要依赖操作系统为其提供系统调用。

Rust 语言的优势：[Rust ](http://www.rust-lang.org/)是一门强调**安全**、**并发**、**高效**的系统编程语言。无 GC，实现内存安全机制、无数据竞争的并发机制、无运行时开销的抽象机制，它声称解决了传统 C 语言和 C++ 语言几十年来饱受诟病的内存安全问题，同时还保持了很高的运行效率、很深的底层控制、很广的应用范围，在系统编程领域具有强劲的竞争力和广阔的应用前景。

**高效性**：Rust 无 GC，无 VM，无解释器，具有极小的运行时开销，能充分高效利用CPU和内存等系统资源。

[以下为几门语言的性能对比](https://github.com/famzah/langs-performance)

| Language                                                     | User   | System | Total  | Slower than (C++) | Language version | Source code                                                  |
| ------------------------------------------------------------ | ------ | ------ | ------ | ----------------- | ---------------- | ------------------------------------------------------------ |
| C++ *([optimized with -O2](http://gcc.gnu.org/onlinedocs/gcc-4.4.4/gcc/Optimize-Options.html#Optimize-Options))* | 0.899  | 0.053  | 0.951  | –                 | g++ 6.1.1        | [link](https://github.com/famzah/langs-performance/blob/master/primes.cpp) |
| Rust                                                         | 0.898  | 0.129  | 1.026  | 7%                | 1.12.0           | [link](https://github.com/famzah/langs-performance/blob/master/primes.rs) |
| Java 8 *([non-std lib](https://blog.famzah.net/2010/07/01/cpp-vs-python-vs-perl-vs-php-performance-benchmark/#comment-4084))* | 1.090  | 0.006  | 1.096  | 15%               | 1.8.0_102        | [link](https://github.com/famzah/langs-performance/blob/master/primes-alt.java) |
| Go                                                           | 2.622  | 0.083  | 2.705  | 184%              | 1.7.1            | [link](https://github.com/famzah/langs-performance/blob/master/primes.go) |
| C++ *(not optimized)*                                        | 2.921  | 0.054  | 2.975  | 212%              | g++ 6.1.1        | [link](https://github.com/famzah/langs-performance/blob/master/primes.cpp) |
| Python 3.5                                                   | 17.950 | 0.126  | 18.077 | 1800%             | 3.5.2            | [link](https://github.com/famzah/langs-performance/blob/master/primes.py) |
| Python 2.7                                                   | 25.219 | 0.114  | 25.333 | 2562%             | 2.7.12           | [link](https://github.com/famzah/langs-performance/blob/master/primes.py) |

**安全性**：Rust 设计上是内存安全的，这也是一大亮点和相较 C/C++的优势。

它不允许**空指针**、**悬空指针**或**数据竞争**。其丰富的**类型系统**和**所有权模型**保证了内存安全和线程安全，使得能够在编译时消除许多类别的错误。也就是说，一段能跑起来的代码大概率是安全的。具体特性如下

- **内存管理**：相比 C++，更强调对象的移动语义，安全快速，强调生命周期，所有权等
- **智能指针**：通过智能指针，如 `Box<T>` 来控制存放在**堆**内存中的类型为 `T` 的值；Rust 的智能指针功能丰富
- **类型安全**：对一些基本类型的行为进行了限制，较少甚至消除**语义不明**确行为。
- **错误处理**：使用 `Option<T>` 解决空指针问题；针对可恢复和不可恢复错误有不同处理。

**生产力**：Rust 有内容详尽的**文档**以及开放、友好、高效的**开源社区**。并且有开放的开发**工具链**。

- 集成的包管理工具 cargo 。
- 编译器能提供有效的错误提示和修正信息，减少了 debug 的时间。
- 自动格式化程序 clippy 规定了代码格式，减少了团队磨合统一标准的时间。
- 支持单元测试，不用引入测试框架。

## 3 架构设计

### 3.1 问题分析

- 将内核任务转换为独立的用户空间服务器需要额外的通信，因为服务器必须与内核（也可能与其他服务器）协作才能执行其工作。因此需要额外的上下文切换，并将引入一些性能开销。下面是一些可选的方案：
  - 测量典型请求-响应序列所需的时间，以了解性能损失。（考虑计算密集型和 I/O 密集型场景）
  - 让 Server 的地址空间重合，避免在 Server 之间进行数据交互的地址空间切换。
  - 在计算密集型场景下，禁用时间片轮转的滴答中断。
- rCore 在实现 IPC 之前就已经有了文件的抽象，但是我们的微内核设计中的 IPC 原语应当不依赖于文件的抽象。下面是一些可选的方案：
  - 底层 Basic IPC 提供 `sys_recv` 和 `sys_send` 系统调用，使用 L4 的 Endpoint 设计模式，以进程 PID 替代文件标识符，实现一个基础的 IPC 模型。这里要做的是从 seL4 从 C 到 Rust 的移植。
  - 提供 Basic IPC 和基于文件抽象的 IPC 两套控制流，根据当前文件系统是否可用来实现灵活的 IPC 模式。
  - 参考混合内核，提供一个缺省文件系统。当上位文件系统不存在时，这个文件系统生效，两个文件系统接口统一。

### 3.2 设计方案

首先给出我们微内核操作系统的整个设计结构：

![image-20220629104124805](./pic/image-20220629104124805.png)

Rust SBI 介于底层硬件和内核之间，是我们内核的底层执行环境。

基于 3.1 节分析的问题，综合考虑，本项目选择如下解决办法：

- 针对 IPC 开销较大的问题，我们：
  - 首先考虑计算密集型和 I/O 密集型场景，分别测量典型请求-响应序列所需的时间，以了解性能损失。
  - 其次将决定权交给用户，让用户根据自己的操作系统要求（从安全、性能和即时性方面考虑）自选安装模式。在不联网的纯计算场景下，可不考虑一些安全性问题，让各 Server 的地址空间重合。
- 针对如何修改 rCore 的 IPC 问题，我们：
  - 首先在底层 Basic IPC 提供 `sys_recv` 和 `sys_send` 系统调用，使用 L4 的 Endpoint 设计模式，以进程 PID 替代文件标识符，实现一个基础的 IPC 模型。
  - 其次，提供 rCore 本身的 easy-fs 作为缺省文件系统。当上位文件系统不存在时，这个文件系统生效。并且在内核中提供初始化的 easy-fs，一旦检测到现有文件系统损坏，又没有可用文件系统时，从备份恢复 easy-fs。

### 3.3 技术路线

#### 3.3.1 Bare-metal

Bare-metal 指的是可以不依赖操作系统运行的可执行文件。因为要写一个 Rust 微内核，我们第一步就需要写出能够在裸机上直接运行的 bare-metal 可执行文件，这在很多教程里都有提及，本项目主要参考 rCore。在写出一个 bare-metal 可执行文件后，逐步往里面添加所需的功能。

例如，逐渐为内核支持函数库的形态，基于 `RustSBI` 完成输出及关机等。

在完成了基本的页表机制和任务调度模块后，即可实现进程模块。

#### 3.3.2 进程调度和 IPC

进程调度和 IPC 是我们第一个主要添加的内容。高性能和多任务并发的支持是本项目内核计划实现的两个特性，因此进程调度和 IPC 显得尤为重要。

Rust 语言本身提供对并发的支持。对于单进程应用而言，Rust 中的关键字 `async` 和 `await` 可以让编译器将异步代码用状态机的形式转写成无栈协程，同时有栈协程也可以由用户态的运行时实现，这可以使得我们专注于进程调度的实现，而将线程管理的任务分摊到用户态，通过提供相应的用户态库来方便编写多线程程序。

我们自己的进程调度和 IPC 模块包括常规的进程管理，创建进程、销毁进程、进程等待；进程调度相关的算法实现；进程间通信，上下文切换的高效实现。

部分模块需要参考或改写其他项目已有的设计（seL4、Redox）。

#### 3.3.3 信号量机制和多线程

要实现多线程机制，首先需要实现信号量机制，以管理互斥资源和实现同步。

在线程的眼里，信号量是一种每个线程能看到的共享资源，且可以存在多个不同信号量来合理使用不同的资源。所以我们可以把信号量也看成四一种资源，可放在一起让进程来管理。操作系统需要显式地施加某种控制，来确定当一个线程执行 P 操作和 V 操作时，如何让线程睡眠或唤醒线程。

多线程不一定需要操作系统的支持，完全可以在用户态实现。我们在用户态构建一个多线程的的基本执行环境（即线程管理运行时）。 首先分析一个简单的用户态多线程应用的执行过程，然后设计支持这种简单多线程应用的执行环境，包括线程的总体结构、管理线程执行的线程控制块数据结构、以及对线程管理相关的重要函数：线程创建和线程切换。

#### 3.3.4 文件系统等服务

因为是微内核设计，文件系统之类的服务都被隔离成模块。

我们设计的 IPC 提供 Basic IPC 和基于文件抽象的 IPC 两套控制流，根据当前文件系统是否可用来实现灵活的 IPC 模式。为了增强我们微内核操作系统的健壮性，我们以 rCore 框架提供的 easy-fs 文件系统作为微内核服务的的示例。

## 4 实现细节

### 4.1 Rust for OS 的细节

#### 4.1.1 所有权模型和移动语义

Rust 编译器会在编译的时候进行比较严格的借用检查，来确保在编译期就解决掉很多内存不安全问题，例如对于一个对象确保同时只存在一个可变引用或多个不可变引用（相当于只读）。

> 移动语义和拷贝语义是相对于的，移动可以类比为计算机中对文件操作的剪切，而拷贝类似于文件的复制。

而对于一些函数参数或者返回值的传递，移动语义可以大大提高速度（否则需要调用拷贝构造函数创建新的对象和析构函数销毁旧的对象）。

Rust 中的结构体或对象如果没有声明可以被拷贝，默认使用的都是移动语义。

#### 4.1.2 RAII 编程范式

举例来说，在实现地址空间抽象时，`MapType` 描述该逻辑段内的所有虚拟页面映射到物理页帧的同一种方式，它是一个枚举类型，在内核当前的实现中支持两种方式：

```rust
// os/src/mm/memory_set.rs

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

pub enum MapType {
    Identical,
    Framed,
}
```

当逻辑段采用 `MapType::Framed` 方式映射到物理内存的时候， `data_frames` 是一个保存了该逻辑段内的每个虚拟页面和它被映射到的物理页帧 `FrameTracker` 的一个键值对容器 。`BTreeMap` 中，这些物理页帧被用来存放实际内存数据而不是作为多级页表中的中间节点。这用到了 RAII 的思想，将这些物理页帧的生命周期绑定到它所在的逻辑段 `MapArea` 下，当逻辑段被回收之后这些之前分配的物理页帧也会自动地同时被回收。

这样的 RAII 资源管理思想随处可见：

```rust
pub struct PidHandle(pub usize);
```

这样的写法类似 C++ 中的 typedef，但是更加强大：将 usize 封装成了 PidHandle 对象，即使是动态内存分配也可以由 Rust 维护结构的生命周期，自动分配释放。

#### 4.1.3 Rust 其他特性

Rust 的强类型机制结合所有权机制可以保证编译器提供强大的静态分析能力，有良好的代码报错提示和修改建议。

Rust 的宏机制非常灵活：利用宏我们可以实现一些在原本 Rust 语法中比较麻烦的事情，甚至是定义自己的 DSL，例如如下是用户库的 `println!` 宏：

```rust
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
```

（Rust 从语言设计的角度考量并不支持函数重载）

### 4.2 rCore 框架的整合

> **Q**：为什么选择 rCore 作为框架，而不选择 Redox ？
>
> **A**：经过小组前期调研发现，Redox 系统虽然声称其是一个微内核设计的操作系统，但从实际来看，其内部功能过于臃肿，代码量庞大，不适合作为典型的微内核设计的操作系统来学习整合。相反，rCore 作为一个教学项目，从最简单的批处理系统一步步自底向上，可以根据需要随时增改代码，在我们的实现选取上具有较大的灵活性和易用性。此外，rCore 基于 RISC-V 架构，其在汇编代码读写方面具有简单易懂的优势，而且也有相应的 Qemu 虚拟机支持。

本项目基础框架为 rCore Ch5。拥有了基于页表的内存管理，简易的进程机制以及一个操作系统内核的最小框架。

首先整个 OS 的抽象自顶向下为，应用程序通过函数调用来实现一些复杂的功能；用户库提供这些函数调用，其内部又是通过系统调用向操作系统发出请求，而操作系统同样有 syscall，在本项目中由 Rust SBI 提供的服务完成；更底层的则是由硬件实现，不赘述。

其次简要介绍内存布局。



![../_images/kernel-as-low.png](https://rcore-os.github.io/rCore-Tutorial-Book-v3/_images/kernel-as-low.png)

对于**内核态**而言，内核地址空间被恒等映射至物理地址空间，使得能够方便地访问内核的各个段。

关于动态内存分配，Rust语言在 `alloc` crate 中设定了一套简洁规范的接口，只要实现了这套接口，内核就可以很方便地支持动态内存分配。

```rust
use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}
```

实现了这样一套接口后，后续就可以方便地使用 Vec、String 等容器。

![../_images/app-as-full.png](https://rcore-os.github.io/rCore-Tutorial-Book-v3/_images/app-as-full.png)

对于**用户态**，不同段通过设置不同的访问权限实现了隔离。通过在最高的虚拟页面设置跳板，其中跳板是一段只读的代码，不同的应用程序的跳板页面均映射到相同的物理内存，通过跳板程序实现内核态和用户态的上下文切换。

真实的 CPU 在内存映射机制中除了 MMU，往往还需要 TLB 以加速地址转换。为确保 MMU 的地址转换能够及时与 satp 寄存器的修改同步，需要立即使用 `sfence.vma` 指令将 TLB 清空，这样 MMU 就不会看到 TLB 中已经过期的键值对了。

关于进程，重点关注 **TCB**：

```rust
pub struct TaskControlBlock {
    // immutable
    pub pid: PidHandle,
    pub kernel_stack: KernelStack,
    // mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
    pub trap_cx_ppn: PhysPageNum,
    pub base_size: usize,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub memory_set: MemorySet,
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Arc<TaskControlBlock>>,
    pub exit_code: i32,
}
```

TCB 主要分两部分，一部分为不可变的元数据：进程标识符 `PidHandle` 和内核栈 `KernelStack`；另一部分为在运行过程中可能发生变化的元数据。

- `trap_cx_ppn`：应用地址空间中的 Trap 上下文被放在的物理页帧的物理页号。
- `base_size`：应用数据仅有可能出现在应用地址空间低于 `base_size` 字节的区域中。借助它我们可以清楚的知道应用有多少数据驻留在内存中。
- `task_cx`：将暂停的任务的任务上下文保存在任务控制块中。
- `task_status`：当前进程的执行状态。
- `memory_set`：应用地址空间。
- `parent`：指向当前进程的父进程（如果存在的话）。
- `children`：将当前进程的所有子进程的任务控制块以 `Arc` 智能指针的形式保存在一个向量中，这样才能够更方便的找到它们。
- `exit_code`：退出码，当进程调用 exit 系统调用主动退出或者执行出错由内核终止的时候，它的会被内核保存在它的任务控制块中，并等待它的父进程通过 waitpid 回收它的资源的同时也收集它的 PID 以及退出码。

### 4.3 进程调度和 IPC

#### 4.3.1 进程调度

在本操作系统中，我们默认采取的进程调度方式是**时间片轮转**（RR），用户也可以在安装时改变策略，在保证安全性的前提下禁用时钟中断以获取性能的提升。在 rCore 的框架下，已经实现了时钟中断的机制。

在 RISC-V 64 架构上，计数器 `mtime` 保存在一个 64 位的 CSR 中，它用来统计处理器自上电以来经过了多少个内置时钟的时钟周期。另外一个 64 位的 CSR `mtimecmp` 的作用是：一旦计数器 `mtime` 的值超过了 `mtimecmp`，就会触发一次时钟中断。这使得我们可以方便的通过设置 `mtimecmp` 的值来决定下一次时钟中断何时触发。

首先需要启用该计数器

```rust
set_clear_csr!(
    /// Supervisor Timer Interrupt Enable
    , set_stimer, clear_stimer, 1 << 5);
```

运行在 M 特权级的 RustSBI 已经预留了相应的接口，rCore 框架通过调用它们来间接实现计时器的控制。

```rust
///get current time
pub fn get_time() -> usize {
    time::read()
}
/// get current time in microseconds
pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}
/// set the next timer interrupt
pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
```

这些被封装在 `timer` 模块中

这样，在触发时钟中断时，转到进程调度服务程序，由它来设置下一个时钟中断的触发，并选择下一个要调度到核上进行的进程

```rust
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
```

#### 4.3.2 IPC

> **Q**：为什么没有在实现的时候参考快速路径（fastpath）的思路 ？
>
> **A**：主要有两方面的考虑。一方面，快速路径（fastpath）与 seL4 的设计具有较大的依赖性，不是一个通用的做法，很难直接移植到我们设计的各类系统调用上。调研时在前人的相关项目（如x-qwq）中，我们发现也是采取了类似的决策——没有将 fastpath 移植到 Rust。另一方面，根据调研，L4 系统设计中的 fastpath 通常需要在汇编中实现以获得最佳性能。seL4 团队在 C 中实现了 fastpath。为获得与汇编实现下类似的性能，他们反复检查 C 编译器的输出以寻找最佳代码。在此过程中他们发现，在专家程序员的充分指导下，GCC（至少在 ARM 上）能够生成与最佳手动优化汇编代码一样快的代码。这表明不再需要装配快速路径。

IPC 在设计上参考了 L4 的实现。具体来说 L4 通过端点（Endpoint）进行。端点可以被认为是一个邮箱，发送者和接收者通过该邮箱通过握手交换消息。任何拥有 Send 能力的人都可以通过 Endpoint 发送消息，任何拥有 Receive 上限的人都可以接收消息。这意味着每个端点可以有任意数量的发送者和接收者。特别是，无论有多少线程尝试从 Endpoint 接收，特定消息仅传递给一个接收者（队列中的第一个接收者）。

在调用时，send-only 操作不返回成功指示，只发送 IPC 系统调用 `Send`，从而实现单向数据传输。send-only 不能用于接收任何信息。结果状态，指示消息是否已被传递，将构成反向通道：接收者可以使用结果状态向发送者发送信息。这将导致允许未经能力明确授权的信息流，不符合设计。(可以将这一点看作是特性)

IPC Channel：

```rust
lazy_static! {
    ///Init IPC
    pub static ref IPC_CHANNEL: UPSafeCell<Vec<IpcMessage>> = unsafe {UPSafeCell::new(Vec::new())};
}
```

根据 IPC Message 和 Request 给出 send 和 recv 方法，操作 Channel：

```rust
pub struct IpcMessage {
    from_pid: usize,
    to_pid: usize,
    message: usize,
    size: usize,
}

pub struct IpcRequest {
    pid: usize,
    buffer: usize,
    size: usize,
}
```

### 4.4 信号量和互斥锁

为了保障进程交替执行过程中对共享数据的正确度写，我们也实现了锁机制

由于系统调用过程中访问共享数据不会被打断，所以我们将访问共享数据封装成 syscall，类似 unix 的互斥锁

对应提供 acquire 和 get, add, release 方法

```rust
///return lock contained value, 0 is default
pub fn lock_get(id: usize) -> usize {
    ......
}

///set lock contained value
pub fn lock_set(id: usize, val: usize) {
    ......
}

///amo add
pub fn lock_add(id: usize, val: isize) -> isize {
    ......
}

///release lock
pub fn lock_release(id: usize) {
    ......
}
```

可以作为信号量使用，我们在用户库提供了封装好的 Wait 和 Signal 操作：

```rust
pub fn lock_wait(id: usize) {
    while sys_lock_add(id, usize::MAX) == -1 {
        yield_();
    }
}

pub fn lock_signal(id: usize) {
    lock_add(id, 1);
}
```

wait 操作需要注意保证原子性。

### 4.5 多线程

简单地说，线程是进程的组成部分，进程可包含1 – n个线程，属于同一个进程的线程共享进程的资源，基本的线程由线程ID、执行状态、当前指令指针(PC)、寄存器集合和栈组成。线程是可以被操作系统或用户态调度器独立调度（Scheduling）和分派（Dispatch）的基本单位。

因此具有如下基本结构：

```rust
struct Task {
    id: usize,
    stack: Vec<u8>,
    ctx: TaskContext,	// 当前指令指针(PC)和通用寄存器集合
    state: State,
}
```

可以由 `yield` 使得线程主动让出资源：

```rust
fn t_yield(&mut self) -> bool {
    let mut pos = self.current;

    // 寻找就绪
    while self.tasks[pos].state != State::Ready {
        pos += 1;
        if pos == self.tasks.len() {
            pos = 0;
        }
        if pos == self.current {
            return false;
        }
    }
	
    // 如果不为空
    if self.tasks[self.current].state != State::Available {
        self.tasks[self.current].state = State::Ready;
    }

    self.tasks[pos].state = State::Running;
    let old_pos = self.current;
    self.current = pos;

    unsafe {
        switch(&mut self.tasks[old_pos].ctx, &self.tasks[pos].ctx);
    }

    !self.tasks.is_empty()
}
```

### 4.6 文件系统等服务

考虑微内核设计，在内核中并没有文件系统，而只预留了一套接口。外部提供的文件系统只需与之相适配即可。

由于有了文件系统的支持，可以方便地进行管理，因此在用户程序中，一开始的 `initproc` 除了会启动 `user_shell` 外还会启动文件系统，这样后续操作能更加便利。具体尝试过的文件系统有 rCore 的 easy-fs 和 Redox 的 initfs。

## 5 实现效果和测试

我们编写的内核将主要在 Qemu 模拟器上运行来检验其正确性。这样做主要是为了方便快捷，只需在命令行输入一行命令即可让内核跑起来。为了让我们的内核能够正确对接到 Qemu 模拟器上，我们首先要对 Qemu 模拟器有一定的了解。我们使用软件 `qemu-system-riscv64` 来模拟一台 64 位 RISC-V 架构的计算机，它包含一个 CPU 、一条物理内存以及若干 I/O 外设。

主要的实现效果建议查看附件中的视频，这里截图以示效果。

测试用户态程序：send（fork 子进程之后传递消息）

![img](pic/U7Q6Q[NK4S4$%2}Y9XUX.png)

测试信号量机制（带锁读写）：

![image-20220702231650901](pic/image-20220702231650901.png)

测试无锁读写（会发生一些错乱）：

![image-20220702231848126](pic/image-20220702231848126.png)

测试文件系统接口：

![image-20220702231838864](pic/image-20220702231838864.png)

测试核酸检测模拟程序：

![image-20220702232057683](pic/image-20220702232057683.png)

## 6 总结和展望

### 6.1 本项目的总结

从前期的调研来看，国内暂时没有太多对微内核操作系统的研究和实践，对微内核的认知大多数体现在“微内核、宏内核、混合内核”三者对比时，其余设计问题没有深入的讨论。本项目试图从微内核架构的基本原理开始，利用一些已有的项目，自底向上组织一个简单而功能相对完善的微内核操作系统。

在这个过程中，小组成员激发了自己思想的火花，针对各类从宏内核转换成微内核的设计提出了自己好的想法。但是遗憾的是，这之中只有一小部分被本项目采纳，如用户根据使用场景开关中断。另外有一些想法，例如针对文件系统和其他 APP 通信缓慢的问题，提出的互通地址空间的想法，则是因为时间有限，无法做出很稳定的 demo。另外还有一些想法，则是在性能、安全和鲁棒性的三方妥协下被否决了。这也从一个方面体现了 OS 设计的复杂性，需要在许多可能的因素之间寻求一种权衡。

本项目对微内核设计的 Basic IPC 进行了一种具体的设计，即通过 L4 “Endpoint” 的思想来收发内核消息。在一些场景下（如计算密集型场景），这样的简单设计特别适合。同时，为了显示 Basic IPC 的应用，我们在其上实现了信号量机制和用户态多线程机制。这些机制在我们测试环境下运转良好，初步体现了微内核操作系统的可行性。

同时我们也认识到，如果是面向嵌入式设备，一个微内核的操作系统可能不需要文件系统等服务就可以运转良好。但是如果是更上层的一些应用场景，文件抽象将不得不被引入。因此我们将文件系统的 APP 形式作为默认模块（这里是 easy-fs），在 init 时就打开，但是实际上和内核是运行在两个不同的 Mode 下。

值得一提的是，我们的项目是在 rCore 的框架上的改动。rCore 本身是一个宏内核设计，但是由于其自底向上的设计，我们裁剪了其一个片段作为基础框架。这个框架中包括一个简陋的 user shell，这是基于 getchar 系统调用来完成的，没有任何分词设计。我们在这个简陋的 shell 上进行了修改，使其能够执行与文件抽象有关的一些经典命令，如 touch、cat 等。

### 6.2 本项目的不足之处

- 我们没有找到突破现有微内核 IPC 瓶颈的根本方法，只是通过引入一些 trade off 来提高性能。

- IPC 没有经过大量进程同时工作的测试，可能在这种情况下会暴露出一些问题。

### 6.3 对未来的展望

对于我们现有的微内核操作系统设计，有如下项可做一些改进和补充：

- 实现多核支持，设计多核相关测试用例。
- 支持 virtio disk 的中断机制，提高 IO 性能。
- 向本操作系统发起攻击，并提出解决方案等。

长期以来，国内对微内核设计的研究寥寥，这有可能是微内核本身设计上的一些弊端导致的。如果重新考虑我们的一般作业场景，对架构设计提出一些新的思路，可能会更好、更自然。调研中也发现，除了宏内核、微内核和混合内核之外，也有学者提出一些其他的架构。这并不指的是传统微内核设计的失败，反之，对其实现的深刻研究有助于新的、好的设计的涌入，我们期待着那一天。

## 7 小组信息及分工

**组长**：黄瑞轩（PB20111686），负责统筹项目整体的进展与走向，项目架构的设计和刷论文，此外还负责了中期、结题报告的撰写和 PPT 制作。

**组员**：刘良宇（PB20000180），负责内核 Basic IPC、信号量机制和多线程机制的具体实现，此外还参与了中期、结题报告的撰写。

**组员**：叶升宇（PB20111701），负责项目测试用例的编写与机制的完善，并且还完成了代码审计工作，此外还参与了中期、结题报告的撰写。

**组员**：许坤钊（PB20111714），负责前期项目环境的配置，此外还参与了中期报告的撰写。

对大家的通力合作及老师的支持表示衷心的感谢！

## 项目代码树

```
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            69            563            916           5843
JSON                           389              0              0            391
make                             3             32              7            115
TOML                             6             24              9             92
Assembly                         3              3             26             86
Dockerfile                       1              4              5             31
Markdown                         1              8              0             10
-------------------------------------------------------------------------------
SUM:                           472            634            963           6568
-------------------------------------------------------------------------------
 
├── bootloader(内核依赖的运行在 M 特权级的 SBI 实现，本项目中我们使用 RustSBI)
│   └── rustsbi-qemu.bin(可运行在 qemu 虚拟机上的预编译二进制版本)
├── easy-fs(rCore 提供从内核中独立出来的一个简单的文件系统 EasyFileSystem 的实现)
│   ├── Cargo.toml
│   └── src
│       ├── bitmap.rs(位图抽象)
│       ├── block_cache.rs(块缓存层，将块设备中的部分块缓存在内存中)
│       ├── block_dev.rs(声明块设备抽象接口 BlockDevice，需要库的使用者提供其实现)
│       ├── efs.rs(实现整个 EasyFileSystem 的磁盘布局)
│       ├── layout.rs(一些保存在磁盘上的数据结构的内存布局)
│       ├── lib.rs
│       └── vfs.rs(提供虚拟文件系统的核心抽象)
├── initfs(Redox 提供的独立文件系统)
│   ├── Cargo.toml
│   └── src
│       ├── lib.rs(文件系统抽象)
│       ├── types.rs(相关数据结构)
├── LICENSE
├── os(内核实现放在 os 目录下)
│   ├── build.rs(基于应用名的应用构建器)
│   ├── Cargo.toml(内核实现的一些配置文件)
│   ├── Makefile
│   └── src
│       ├── config.rs(内核的一些配置，包括内存管理的相关配置)
│       ├── console.rs(将打印字符的 SBI 接口封装实现格式化输出)
│       ├── drivers(块设备驱动)
│       │   ├── block
│       │   │   ├── mod.rs(将不同平台上的块设备全局实例化为 BLOCK_DEVICE 提供给其他模块使用)
│       │   │   └── virtio_blk.rs(Qemu 平台的 virtio-blk 块设备)
│       │   └── mod.rs
│       ├── fs(文件系统接口)
│       │   ├── inode.rs(OSInode)
│       │   ├── mod.rs
│       │   └── stdio.rs(标准输入输出)
│       ├── entry.asm(设置内核执行环境的的一段汇编代码)
│       ├── lang_items.rs(需要提供给 Rust 编译器的一些语义项，目前包含内核 panic 时的处理逻辑)
│       ├── link_app.S(构建产物，由 os/build.rs 输出)
│       ├── linker-qemu.ld(控制内核内存布局的链接脚本以使内核运行在 qemu 虚拟机上)(修改：将跳板页引入内存布局)
│       ├── main.rs(内核主函数)
│       ├── mm(内存管理)
│       │   ├── address.rs(物理/虚拟 地址/页号的 Rust 抽象)
│       │   ├── frame_allocator.rs(物理页帧分配器)
│       │   ├── heap_allocator.rs(内核动态内存分配器)
│       │   ├── memory_set.rs(引入地址空间 MemorySet 及逻辑段 MemoryArea 等)
│       │   ├── mod.rs(定义了 mm 模块初始化方法 init)
│       │   └── page_table.rs(多级页表抽象 PageTable 以及其他内容)
│       ├── sbi.rs(调用底层 SBI 实现提供的 SBI 接口)
│       ├── sync(同步子模块 sync ，目前唯一功能是提供 UPSafeCell)
│       │   ├── mod.rs
│       │   └── up.rs(包含 UPSafeCell，它可以帮助我们以更 Rust 的方式使用全局变量)
│       ├── syscall(系统调用子模块 syscall)
│       │   ├── fs.rs(包含文件 I/O 相关的 syscall)
│       │   ├── mod.rs(提供 syscall 方法根据 syscall ID 进行分发处理)
│       │   └── process.rs(包含任务处理相关的 syscall)
│       ├── task(task 子模块，主要负责任务管理)
│       │   ├── context.rs(引入 Task 上下文 TaskContext)
│       │   ├── manager.rs(任务管理器)
│       │   ├── mod.rs(全局任务管理器和提供给其他模块的接口，支持进程)
│       │   ├── pid.rs(进程标识符和内核栈的 Rust 抽象)
│       │   ├── processor.rs(处理器管理结构)
│       │   ├── switch.rs(将任务切换的汇编代码解释为 Rust 接口 __switch)
│       │   ├── switch.S(任务切换的汇编代码)
│       │   └── task.rs(任务控制块 TaskControlBlock 和任务状态 TaskStatus 的定义，支持进程管理机制)
│       ├── timer.rs(计时器相关)
│       └── trap(Trap 相关)
│           ├── context.rs(包含 Trap 上下文 TrapContext)
│           ├── mod.rs(包含 Trap 处理入口 trap_handler，有时钟中断相应处理，基于地址空间，支持进程系统调用)
│           └── trap.S(包含 Trap 上下文保存与恢复的汇编代码)
├── rust-toolchain(控制整个项目的工具链版本)
└── user(应用测例)
├── Cargo.toml
├── Makefile
└── src
    ├── bin(基于用户库 user_lib 开发的应用，每个应用放在一个源文件中)
    │   ├── covid.rs(生产者消费者测试，灵感来源于期末考试)
    │   ├── exit.rs(exit 测试)
    │   ├── hello_world.rs(经典 hello world)
    │   ├── huge_write.rs(文件写测试)
    │   ├── initproc.rs(用户运行的 init 程序)
    │   ├── lock.rs(锁测试)
    │   ├── ls.rs(打印当前目录)
    │   ├── nonlock.rs(无锁测试)
    │   ├── send.rs(IPC测试)
    │   ├── sleep.rs(任务调度测试)
    │   ├── thread.rs(多线程测试)
    │   ├── user_shell.rs(shell)
    │   └── yield.rs(主动移交控制)
    ├── console.rs(通内核态，用户态实际调用内核态)
    ├── lang_items.rs
    ├── lib.rs(用户库 user_lib)
    ├── linker.ld(应用的链接脚本，将所有应用放在各自地址空间中固定的位置)
    └── syscall.rs(包含 syscall 方法生成实际用于系统调用的汇编指令，各个具体的 syscall 都是通过 syscall 来实现的)
```

## 参考资料

[1] Rustpi: A Rust-powered Reliable Micro-kernel Operating System, Yuanzhi Liang, Lei Wang, Siran Li, Bo Jiang School of Computer Science and Engineering, Beihang University.

[2] NileOS: A Distributed Asymmetric Core-Based Micro-Kernel for Big Data Processing, AHMAD EL-ROUBY, etc., The American University in Cairo, New Cairo 11835, Egypt.

[3] Verified Software: Theories, Tools, and Experiments, 7th International Conference, VSTTE 2015 San Francisco, CA, USA, July 18–19, 2015 Revised Selected Papers.

[4] Comprehensive Formal Verification of an OS Microkernel, GERWIN KLEIN, etc., Sydney , Australia

[5] Toward a True Microkernel Operating System, Retrieved 22 June 2015.

[6] [Advantages and disadvantages of Windows operating system](https://www.geeksforgeeks.org/advantages-and-disadvantages-of-windows-operating-system/)

[7] [Ownership is Theft: Experiences Building an Embedded OS in Rust](https://patpannuto.com/pubs/levy15ownership.pdf)

[8] [Rust OS comparison](https://github.com/flosse/rust-os-comparison)

[9] [如何用 Rust 编写一个 Linux 内核模块](https://developer.51cto.com/article/670600.html)

[10] [开源项目：使用 Rust 写一个兼容 Linux 的内核](https://jishuin.proginn.com/p/763bfbd6be97)

[11] [CATTmew: Defeating Software-only Physical Kernel Isolation](http://arxiv.org/abs/1802.07060v4)

[12] [rCore-Tutorial-Book 第三版](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)

[13] [Extreme High Performance Computing or Why Microkernels Suck](https://www.kernel.org/doc/ols/2007/ols2007v1-pages-251-262.pdf) 

[14] Blackham and G. Heiser. 2012. Correct, Fast, Maintainable – Choose Any Three!. In Proceedings of the 3rd Asia-Pacific Workshop on Systems (APSys).
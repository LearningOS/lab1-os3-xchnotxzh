

# 功能总结

实验在多道程序执行环境的基础上，实现了

1. 分级隔离执行环境
   1. 实现内容 -- 让app在低特权级运行，让kernel在高特权级运行，并提供基于trap的高特权级服务（包括系统调用服务）。
   2. 实现策略 -- 基于特权级机制实现分级执行，基于trap机制实现控制流和特权级转换以支持系统调用服务。
2. 任务调度机制
   1. 实现内容 -- 抢占调度和协作调度
   2. 实现策略 -- 先实现任务切换，然后在此基础上封装成抢占调度和协作调度
3. 获取当前任务信息的系统调用
   1. 实现内容 -- 获取当前任务的
      1. 任务状态 -- 任务运行状态
      2. 系统调用次数 -- 每个系统调用的次数，包括本系统调用。
      3. 任务执行时间 -- 距离任务第一次被调度时刻的时长，可包含等待重新调度的时间。
   2. 实现策略
      1. 任务状态统计 -- 记录在当前任务控制块中，随着任务调度更新。
      2. 系统调用次数统计 -- 不能随着调度消失，所以记录到全局任务管理器中。在进trap统一处理处，累计系统调用次数。
      3. 任务执行时间统计 -- 记录在当前任务控制块中，为了方便统计，将首次开始任务也看成从暂停而来，则可针对暂停计时。
         1. 若是第一次开始 -- 记”上次暂停时间”为”当前时间”。
         2. 暂停 -- “总运行时间” 累加 ”当前时间” 减去 ”上次暂停时间”，记”上次暂停时间”为”当前时间” 。
         3. 退出 -- “总运行时间” 累加 ”当前时间” 减去 ”上次暂停时间”。



# 问答题

## 1

### 问题

正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 [Rust 三个 bad 测例 (ch2b_bad_*.rs)](https://github.com/LearningOS/rust-based-os-comp2022/tree/main/user/src/bin) ， 注意在编译时至少需要指定 `LOG=ERROR` 才能观察到内核的报错信息) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

### 解答

RustSBI version 0.3.0-alpha.4, adapting to RISC-V SBI v1.0.0 的报错信息如下：

     ch2b_bad_address的错误是访问非法地址，因为访问物理地址为0的非法地址
    [ERROR] [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x80400408, core dumped.  
     ch2b_bad_instructions的错误是执行非法指令，因为在U态使用sret的S态指令
    [ERROR] [kernel] IllegalInstruction in application, core dumped.  
     ch2b_bad_address的错误是执行非法指令，因为在U态访问sstatus的S态寄存器
    [ERROR] [kernel] IllegalInstruction in application, core dumped.  

## 2

深入理解 [trap.S](https://github.com/LearningOS/rust-based-os-comp2022/blob/main/os3-ref/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下问题:

### 2.1

#### 问题

L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。

#### 解答

a0保存了上一次调度时被换出的Trap上下文的地址。

__restore使用场景

1. 内核初始化完成后，从内核进入用户态来执行app。
2. 处理完Trap后返回app继续执行。

### 2.2

#### 问题

 L46-L51：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

```assembly
ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2
```

#### 解答

- sstatus -- 记录Trap发生时的内核状态，其SPP等字段标识特权级，用于在处理完Trap之后正确的返回到用户态
- sepc -- 记录Trap发生时的最后指令地址，Trap处理完返回时会用sret跳回这个地址继续执行app
- sscratch -- 进入用户态时，保存内核栈地址，以便将来restore时换栈

### 2.3

#### 问题

L53-L59：为何跳过了 `x2` 和 `x4`？

```assembly
ld x1, 1*8(sp)
ld x3, 3*8(sp)
.set n, 5
.rept 27
   LOAD_GP %n
   .set n, n+1
.endr
```

#### 解答

- x2是sp，跳过是因为，要留在后面换栈。
- x4是tp，跳过是因为，目前用不到。

### 2.4

#### 问题

L63：该指令之后，sp 和 sscratch 中的值分别有什么意义？

#### 解答

换栈指令，该指令之后，sp是用户栈顶地址，sscratch是内核栈顶地址。

### 2.5

#### 问题

__restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

#### 解答

状态切换指令是sret。

该指令将特权级设为sstatus的SPP字段，然后跳转到sepc记录的Trap前的指令地址继续执行。

### 2.6

#### 问题

L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？

#### 解答

换栈指令，该指令之后，sp是内核栈顶地址，sscratch是用户栈顶地址。

### 2.7

#### 问题

从 U 态进入 S 态是哪一条指令发生的？

#### 解答

ecall。

# 建议

本章的实验内容太多且不模块化，将分级隔离执行环境和任务调度混到一起了。

应将分级隔离执行环境的内容放到之前的章节。
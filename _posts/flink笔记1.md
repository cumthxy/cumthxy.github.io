---
layout: post
title: flink笔记
categories: flink
description: flink笔记总结
keywords: flink
---

## 1.flink运行架构



Flink运行时架构主要分为四个不同组件

- 作业管理器**（JobManager）**
- 资源管理器**（ResourceManager）**
- 任务管理器**（TaskManager）**
- 并发器**（Dispatcher）**





####  作业管理器

 控制一个应用程序执行的主进程，也就是说，每个应用程序都会被一个不同的JobManager所控制执行。

 JobManager会先接收到要执行的应用程序，这个应用程序会包括：

- 作业图（JobGraph）
- 逻辑数据流图（logical dataflow graph）
- 打包了所有的类、库和其它资源的JAR包。

 JobManager会把JobGraph转换成一个物理层面的数据流图，这个图被叫做“执行图”（ExecutionGraph），包含了所有可以并发执行的任务。

 **JobManager会向资源管理器（ResourceManager）请求执行任务必要的资源，也就是任务管理器（TaskManager）上的插槽（slot）。一旦它获取到了足够的资源，就会将执行图分发到真正运行它们的TaskManager上**。

 在运行过程中，JobManager会负责所有需要中央协调的操作，比如说检查点（checkpoints）的协调。

### 资源管理器

 主要负责管理任务管理器（TaskManager）的插槽（slot），TaskManger插槽是Flink中定义的处理资源单元。

 Flink为不同的环境和资源管理工具提供了不同资源管理器，比如YARN、Mesos、K8s，以及standalone部署。

 **当JobManager申请插槽资源时，ResourceManager会将有空闲插槽的TaskManager分配给JobManager**。如果ResourceManager没有足够的插槽来满足JobManager的请求，它还可以向资源提供平台发起会话，以提供启动TaskManager进程的容器。

 另外，**ResourceManager还负责终止空闲的TaskManager，释放计算资源**。



### 任务管理器



Flink中的工作进程。通常在Flink中会有多个TaskManager运行，每一个TaskManager都包含了一定数量的插槽（slots）。**插槽的数量限制了TaskManager能够执行的任务数量**。

 启动之后，TaskManager会向资源管理器注册它的插槽；收到资源管理器的指令后，TaskManager就会将一个或者多个插槽提供给JobManager调用。JobManager就可以向插槽分配任务（tasks）来执行了。

 **在执行过程中，一个TaskManager可以跟其它运行同一应用程序的TaskManager交换数据**。

### 分发器



可以跨作业运行，它为应用提交提供了REST接口。

 当一个应用被提交执行时，分发器就会启动并将应用移交给一个JobManager。由于是REST接口，所以Dispatcher可以作为集群的一个HTTP接入点，这样就能够不受防火墙阻挡。Dispatcher也会启动一个Web UI，用来方便地展示和监控作业执行的信息。

 *Dispatcher在架构中可能并不是必需的，这取决于应用提交运行的方式。*



### 任务提交流程



![image-20210311151654895](https://gitee.com/cumthxy/Image/raw/master/img/image-20210311151654895.png)



 具体地，如果我们将Flink集群部署到YARN上，那么就会有如下的提交流程：

![image-20210311151754472](https://gitee.com/cumthxy/Image/raw/master/img/image-20210311151754472.png)

1. Flink任务提交后，Client向HDFS上传Flink的Jar包和配置
2. 之后客户端向Yarn ResourceManager提交任务，ResourceManager分配Container资源并通知对应的NodeManager启动ApplicationMaster
3. ApplicationMaster启动后加载Flink的Jar包和配置构建环境，去启动JobManager，之后**JobManager向Flink自身的RM进行申请资源，自身的RM向Yarn 的ResourceManager申请资源(因为是yarn模式，所有资源归yarn RM管理)启动TaskManager**
4. Yarn ResourceManager分配Container资源后，由ApplicationMaster通知资源所在节点的NodeManager启动TaskManager
5. NodeManager加载Flink的Jar包和配置构建环境并启动TaskManager，TaskManager启动后向JobManager发送心跳包，并等待JobManager向其分配任务。

### 任务调度原理



![](https://gitee.com/cumthxy/Image/raw/master/img/image-20210311152545733.png)



1. 客户端不是运行时和程序执行的一部分，但它用于准备并发送dataflow(JobGraph)给Master(JobManager)，然后，客户端断开连接或者维持连接以等待接收计算结果。而Job Manager会产生一个执行图(Dataflow Graph)
2. 当 Flink 集群启动后，首先会启动一个 JobManger 和一个或多个的 TaskManager。由 Client 提交任务给 JobManager，JobManager 再调度任务到各个 TaskManager 去执行，然后 TaskManager 将心跳和统计信息汇报给 JobManager。TaskManager 之间以流的形式进行数据的传输。上述三者均为独立的 JVM 进程。
3. Client 为提交 Job 的客户端，可以是运行在任何机器上（与 JobManager 环境连通即可）。提交 Job 后，Client 可以结束进程（Streaming的任务），也可以不结束并等待结果返回。
4. JobManager 主要负责调度 Job 并协调 Task 做 checkpoint，职责上很像 Storm 的 Nimbus。从 Client 处接收到 Job 和 JAR 包等资源后，会生成优化后的执行计划，并以 Task 的单元调度到各个 TaskManager 去执行。
5. TaskManager 在启动的时候就设置好了槽位数（Slot），每个 slot 能启动一个 Task，Task 为线程。从 JobManager 处接收需要部署的 Task，部署启动后，与自己的上游建立 Netty 连接，接收数据并处理。







### TaskManger 于Slots

- 考虑到Slot分组，所以实际运行Job时所需的Slot总数 = 每个Slot组中的最大并行度。(自己设置的最大任务并行度)

  eg（1，1，2，1）,其中第一个归为组“red”、第二个归组“blue”、第三个和第四归组“green”，那么运行所需的slot即max（1）+max（1）+max（2，1） = 1+1+2 = 4

这里不会涉及到CPU的隔离，slot目前仅仅用来隔离task的受管理的内存。

![image-20210311153623580](https://gitee.com/cumthxy/Image/raw/master/img/image-20210311153623580.png)

slot 默认 cpu并行度

- Flink中每一个worker(TaskManager)都是一个**JVM**进程，它可能会在独立的线程上执行一个或多个subtask。

为了控制一个worker能接收多少个task，worker通过task slot来进行控制（一个worker至少有一个task slot）

### Slot 和并行度

1. **一个特定算子的 子任务（subtask）的个数被称之为其并行度（parallelism）**，我们可以对单独的每个算子进行设置并行度，也可以直接用env设置全局的并行度，更可以在页面中去指定并行度。
2. 最后，由于并行度是实际Task Manager处理task 的能力，而一般情况下，**一个 stream 的并行度，可以认为就是其所有算子中最大的并行度**，则可以得出**在设置Slot时，在所有设置中的最大设置的并行度大小则就是所需要设置的Slot的数量。**（如果Slot分组，则需要为每组Slot并行度最大值的和）




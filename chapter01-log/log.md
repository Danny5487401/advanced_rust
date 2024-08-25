<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [tracing](#tracing)
  - [tracing 各个模块](#tracing-%E5%90%84%E4%B8%AA%E6%A8%A1%E5%9D%97)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->


# tracing 

tracing 支持 log 门面库的 API，因此，它既可以作为分布式追踪的 SDK 来使用，也可以作为日志库来使用。

## tracing 各个模块
- tracing  : 作用域内的结构化日志记录和诊断系统。
- tracing_appender : 记录事件和跨度的编写者。也就是将日志写入文件或者控制台。
- tracing_error : 增强错误处理跟踪诊断信息的实用工具。
- tracing_flame : 用于生成折叠堆栈跟踪以生成Inferno火焰图和火焰图表的跟踪订阅者。
- tracing_log : 用于连接标准库日志系统和tracing系统的连接器。
- tracing_subscriber : 用于实现和组成tracing订阅者的工具集合。
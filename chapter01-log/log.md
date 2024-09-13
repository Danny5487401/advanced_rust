<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [tracing](#tracing)
  - [tracing 各个模块](#tracing-%E5%90%84%E4%B8%AA%E6%A8%A1%E5%9D%97)
  - [log 门面](#log-%E9%97%A8%E9%9D%A2)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# tracing

tracing 支持 log 门面库的 API，因此，它既可以作为分布式追踪的 SDK 来使用，也可以作为日志库来使用。

## tracing 各个模块

- tracing: 作用域内的结构化日志记录和诊断系统。
- tracing_appender: 记录事件和跨度的编写者。也就是将日志写入文件或者控制台。
- tracing_error: 增强错误处理跟踪诊断信息的实用工具。
- tracing_flame: 用于生成折叠堆栈跟踪以生成Inferno火焰图和火焰图表的跟踪订阅者。
- tracing_log: 用于连接标准库日志系统和tracing系统的连接器。
- tracing_subscriber: 用于实现和组成tracing订阅者的工具集合。

## log 门面

github.com/rust-lang/log

```rust
// log-0.4.22/src/lib.rs
pub trait Log: Sync + Send {
    // 判断某条带有元数据的日志是否能被记录
    fn enabled(&self, metadata: &Metadata) -> bool;

    // 记录 record 所代表的日志
    fn log(&self, record: &Record);

    // 将缓存中的日志数据刷到输出中，例如标准输出或者文件中
    fn flush(&self);
}
```

log 仅仅是日志门面库，它并不具备完整的日志库功能！因此你无法在控制台中看到任何日志输出

env_logger 实现

```rust
// env_logger-0.9.3/src/lib.rs
impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.matches(record) {
            // Log records are written to a thread-local buffer before being printed
            // to the terminal. We clear these buffers afterwards, but they aren't shrinked
            // so will always at least have capacity for the largest log record formatted
            // on that thread.
            //
            // If multiple `Logger`s are used by the same threads then the thread-local
            // formatter might have different color support. If this is the case the
            // formatter and its buffer are discarded and recreated.

            thread_local! {
                static FORMATTER: RefCell<Option<Formatter>> = RefCell::new(None);
            }

            let print = |formatter: &mut Formatter, record: &Record| {
                let _ =
                    (self.format)(formatter, record).and_then(|_| formatter.print(&self.writer));

                // Always clear the buffer afterwards
                formatter.clear();
            };

            let printed = FORMATTER
                .try_with(|tl_buf| {
                    match tl_buf.try_borrow_mut() {
                        // There are no active borrows of the buffer
                        Ok(mut tl_buf) => match *tl_buf {
                            // We have a previously set formatter
                            Some(ref mut formatter) => {
                                // Check the buffer style. If it's different from the logger's
                                // style then drop the buffer and recreate it.
                                if formatter.write_style() != self.writer.write_style() {
                                    *formatter = Formatter::new(&self.writer);
                                }

                                print(formatter, record);
                            }
                            // We don't have a previously set formatter
                            None => {
                                let mut formatter = Formatter::new(&self.writer);
                                print(&mut formatter, record);

                                *tl_buf = Some(formatter);
                            }
                        },
                        // There's already an active borrow of the buffer (due to re-entrancy)
                        Err(_) => {
                            print(&mut Formatter::new(&self.writer), record);
                        }
                    }
                })
                .is_ok();

            if !printed {
                // The thread-local storage was not available (because its
                // destructor has already run). Create a new single-use
                // Formatter on the stack for this call.
                print(&mut Formatter::new(&self.writer), record);
            }
        }
    }

    fn flush(&self) {}
}
```

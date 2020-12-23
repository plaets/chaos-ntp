use slog::{o,Drain,info,trace,warn,error};
use slog_term;
use slog_async;
use slog_scope;

pub fn setup_logger() -> slog_scope::GlobalLoggerGuard {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    slog_scope::set_global_logger(logger)
}


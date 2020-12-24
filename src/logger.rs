use slog::{o,Drain};
use slog_async;
use slog_scope;

//should i use structured logging? rn i just pack everything into string
pub fn setup_logger() -> slog_scope::GlobalLoggerGuard {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        //TODO: use a custom header when they finally publish the new package version with this
        //.use_custom_header_print(|timestamp, rd, record, use_file_location| {
        //})
        .build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    slog_scope::set_global_logger(logger)
}


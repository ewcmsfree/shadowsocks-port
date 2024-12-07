use time::{macros::format_description, UtcOffset};
use tracing::Level;
use tracing_subscriber::{fmt::time::OffsetTime, EnvFilter};

/// 设置日志打印级别
pub fn set_tracing_subscriber(level: Level) {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(local_time)
        .with_max_level(level)
        .with_thread_names(true)
        .with_thread_ids(true)
        .init();
}

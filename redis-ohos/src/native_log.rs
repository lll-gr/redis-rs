use napi_derive_ohos::napi;
use ohos_hilog_binding::{
    hilog_debug, hilog_error, hilog_info, hilog_warn, set_global_options, LogOptions,
};
use std::collections::HashMap;
use std::panic;
use tracing::{Event, Subscriber};
use tracing_core::Level;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::prelude::*;

static PANIC_HOOK_INITIALIZED: std::sync::Once = std::sync::Once::new();
static TRACING_INITIALIZED: std::sync::Once = std::sync::Once::new();
static HILOG_INITIALIZED: std::sync::Once = std::sync::Once::new();

fn panic_hook(info: &panic::PanicHookInfo) {
    hilog_error!("RUST PANIC: {}", info);
}

#[napi]
pub fn init_panic_hook() {
    PANIC_HOOK_INITIALIZED.call_once(|| {
        panic::set_hook(Box::new(panic_hook));
    });
}

#[napi]
pub fn hilog_global_options(domain: u32, tag: String) {
    HILOG_INITIALIZED.call_once(|| {
        ohos_hilog_binding::forward_stdio_to_hilog();
        set_global_options(LogOptions {
            domain,
            tag: Box::leak(tag.clone().into_boxed_str()),
        })
    });
}

#[napi]
pub fn init_tracing_subscriber() {
    TRACING_INITIALIZED.call_once(|| {
        tracing_subscriber::registry()
            .with(CallbackLayer {
                callback: Box::new(tracing_callback),
            })
            .init();
    });
}

/// 初始化日志系统（一次性完成所有初始化）
///
/// 这个函数会：
/// 1. 设置 panic hook
/// 2. 配置 hilog 全局选项
/// 3. 初始化 tracing subscriber
///
/// **参数：**
/// - `domain`: u32 - hilog domain（建议使用 0xD000000 到 0xD0FFFFF 之间的值）
/// - `tag`: String - hilog tag（建议使用应用名称）
///
/// **注意：** 此函数可以多次调用，但只会初始化一次（幂等性）
///
/// **使用示例（ArkTS）：**
/// ```typescript
/// import { initLogging } from 'libbitwarden_ohos.so';
///
/// // 在应用启动时调用一次
/// initLogging(0xD001000, "MyApp");
/// ```
#[napi]
pub fn init_logging(domain: u32, tag: String) {
    init_panic_hook();
    hilog_global_options(domain, tag);
    init_tracing_subscriber();
}

fn tracing_callback(event: &Event, fields: HashMap<String, String>) {
    let metadata = event.metadata();
    #[cfg(target_env = "ohos")]
    {
        let loc = metadata.target().split("::").last().unwrap();
        match *metadata.level() {
            Level::TRACE => {
                hilog_debug!("[{}] {:?}", loc, fields.values().collect::<Vec<_>>());
            }
            Level::DEBUG => {
                hilog_debug!("[{}] {:?}", loc, fields.values().collect::<Vec<_>>());
            }
            Level::INFO => {
                hilog_info!("[{}] {:?}", loc, fields.values().collect::<Vec<_>>());
            }
            Level::WARN => {
                hilog_warn!("[{}] {:?}", loc, fields.values().collect::<Vec<_>>());
            }
            Level::ERROR => {
                hilog_error!("[{}] {:?}", loc, fields.values().collect::<Vec<_>>());
            }
        }
    }
}

struct CallbackLayer {
    callback: Box<dyn Fn(&Event, HashMap<String, String>) + Send + Sync>,
}

impl<S: Subscriber> Layer<S> for CallbackLayer {
    fn on_event(&self, event: &Event, _ctx: Context<S>) {
        // 使用 fmt::format::FmtSpan 提取字段值
        let mut fields = HashMap::new();
        let mut visitor = FieldCollector(&mut fields);
        event.record(&mut visitor);
        (self.callback)(event, fields);
    }
}

struct FieldCollector<'a>(&'a mut HashMap<String, String>);

impl<'a> tracing::field::Visit for FieldCollector<'a> {
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0.insert(field.name().to_string(), value.to_string());
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0.insert(field.name().to_string(), value.to_string());
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name().to_string(), format!("{:?}", value));
    }
}

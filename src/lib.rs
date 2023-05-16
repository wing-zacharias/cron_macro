macro_rules! codegen_reexport {
    ($name:ident) => {
        pub use cron_macro_codegen::$name;
    };
}

codegen_reexport!(cron);

#[macro_export]
macro_rules! cron_task_run {
    ($($var:ident,)*) => {
        $($var().await;)*
    };
}

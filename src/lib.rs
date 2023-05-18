macro_rules! codegen_reexport {
    ($name:ident) => {
        pub use cron_macro_codegen::$name;
    };
}

codegen_reexport!(cron);

///
/// start run schedule function!
/// usage:
///     
///     #[cron(...)]
///     fn func1() ...
///
///     #tokio::main]
///     async fn main(){
///         ...
///         cron_task_run!(func1,);
///         ...
///     }
///
///
#[macro_export]
macro_rules! cron_task_run {
    ($($var:ident,)*) => {
        $($var().await;)*
    };
}

A simple cron crate for rust project! Base on tokio-cron-scheduler,for example:

```toml
cron_macro = {version = "0.1.3"}
ctrlc = "3.2.5"
tokio = {version = "1.26.0", features = ["full", "macros"]}
tokio-cron-scheduler = "0.9.4"
lazy_static = "1.4.0"
serde = {version = "1.0.158", features = ["derive"]}
serde_json = "1.0.94"
error-chain = "0.12.4"
uuid = {version = "1.3.0", default-features = false, features = ["serde", "v4"]}
regex = "1.8.0"
```

```
json
{
    "task":{
        "cron":"0/2 * * * * *"
    }
}
```

```rust
use cron_macro::{cron, cron_task_run};
use std::fs::read_to_string;
use serde::{Deserialize, Serialize};

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::time::sleep;

lazy_static::lazy_static! {
    static ref GLOBAL_CONFIG: Config = Config::new("config.json");
}

pub struct CfgTask {
    pub cron: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub task: CfgTask,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let contents = read_to_string(path).unwrap();
        serde_json::from_str(&contents).unwrap()
    }
}

#[cron("${GLOBAL_CONFIG.task.cron}")]
pub fn my_func1() {
    println!("my_func1");
}

#[cron("0/3 * * * * *")]
pub fn my_func2() {
    println!("my_func2");
}

pub async fn shutdown_on_ctrl_c() {
    println!("Pressed ctrl-c to exit process!");
    let running = Arc::new(AtomicBool::new(true));
    let running_sig = running.clone();
    let running_ctl = running.clone();
    ctrlc::set_handler(move || running_ctl.store(false, Ordering::Relaxed))
        .expect("set handler error: ctrl-c !");
    while running_sig.load(Ordering::Relaxed) {
        sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    cron_task_run!(my_func1, my_func2,);
    shutdown_on_ctrl_c().await;
}
```

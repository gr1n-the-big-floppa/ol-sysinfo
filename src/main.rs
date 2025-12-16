use clap::Parser;
use log::{debug, error};
use omni_led_api::plugin::Plugin;
use omni_led_api::types::Table;
use sysinfo::{Components, System};
use tokio::time::{Instant, sleep_until, Duration};

const NAME: &str = "SYSINFO";

#[tokio::main]
async fn main() {
    let options = Options::parse();

    let mut plugin = match Plugin::new(NAME, &options.address).await {
        Ok(p) => p,
        Err(err) => {
            error!("Failed to initialize plugin: {err}");
            return;
        }
    };

    run_loop(&mut plugin, &options).await;
}

async fn run_loop(plugin: &mut Plugin, options: &Options) {
    let mut next_tick = Instant::now();

    let mut sys = System::new();
    let mut components = Components::new();

    loop {
        update_temps(plugin, &mut sys, &mut components).await;

        next_tick += Duration::from_secs(options.update_interval.max(1));
        sleep_until(next_tick).await;
    }
}

async fn update_temps(
    plugin: &mut Plugin,
    sys: &mut System,
    components: &mut Components,
) {
    let mut table = Table::default();

    sys.refresh_cpu_all();
    sys.refresh_memory();

    table.items.insert("cpu usage".into(), sys.global_cpu_usage().into());
    table.items.insert("used memory".into(), sys.used_memory().into());
    table.items.insert("total memory".into(), sys.total_memory().into());
    table.items.insert("available memory".into(), sys.available_memory().into());

    components.refresh(false);
    for component in components.list() {
        if let Some(temp) = component.temperature() {
            table.items.insert(component.label().to_owned(), temp.into());
        }
    }

    if let Err(err) = plugin.update(table).await {
        error!("Failed to push update to plugin: {err}");
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Options {
    #[clap(short, long)]
    address: String,

    #[clap(short, long)]
    update_interval: u64,
}

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

    loop {
        update_temps(plugin).await;

        next_tick += Duration::from_secs(options.update_interval);
        sleep_until(next_tick).await;
    }
}

async fn update_temps(plugin: &mut Plugin) {
    let mut table = Table::default();
    let mut sys = System::new_all();

    sys.refresh_all();
    table.items.insert("cpu usage".into(), sys.global_cpu_usage().into());

    table.items.insert("used memory".into(), sys.used_memory().into());
    table.items.insert("total memory".into(), sys.total_memory().into());
    table.items.insert("free memory".into(), sys.free_memory().into());
    table.items.insert("available memory".into(), sys.available_memory().into());

    let components = Components::new_with_refreshed_list();
    for component in components.list() {
        let label = component.label();
        let temp = component.temperature();

        match temp {
            Some(value) => {
                table.items.insert(label.to_owned(), value.into());
            }
            None => {
                debug!("Skipping component '{}': no temperature reported", label);
            }
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

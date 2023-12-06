pub mod api;
use api::check_rabbitmq_node_health;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// rabbitmq hosts split by ";"
    #[arg(long)]
    hosts: String,

    /// rabbitmq hosts port
    #[arg(long, default_value_t = 15672)]
    port: u16,

    /// rabbitmq username with admin permission
    #[arg(long)]
    username: String,

    /// rabbitmq password with admin permission
    #[arg(long)]
    password: String,

    /// interval in seconds between two checks
    #[arg(long, default_value_t = 3)]
    interval: u8,
}

fn main() {
    // init env logger
    env_logger::init();

    let args = Args::parse();
    let mut partitions: Vec<String> = vec![];

    // split hosts
    let hosts = args.hosts.split(';');

    for host in hosts {
        let partition_hosts =
            check_rabbitmq_node_health(host, args.port, &args.username, &args.password);
        partitions.extend(partition_hosts);
    }

    // calulate partitions
    let mut counts: HashMap<&str, u8> = HashMap::new();

    for partition in &partitions {
        *counts.entry(partition.as_str()).or_insert(0) += 1;
    }

    let mut max_count: u8 = 0;
    let mut max_partition = "";
    for (partition, count) in counts {
        if count > max_count {
            max_count = count;
            max_partition = partition;
        }
    }
    println!("partition: {}, count: {}", max_partition, max_count);
}

extern crate structopt;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(help = "Port to scan")]
    pub ports: String,
}

pub fn run(options: &mut Options) {
    let mut ports = String::from(":");
    ports.push_str(&options.ports);

    // `lsof` tends to take a little bit,
    // so why don't we time it?
    let now = std::time::Instant::now();
    let lsof_output = Command::new("lsof")
        .arg("-i")
        .arg(&ports[..])
        .stdout(Stdio::piped())
        .output()
        .expect("Command `lsof` failed; try checking if it is installed");
    let lsof_time = now.elapsed().as_secs();

    println!(
        "\nProcess and address information collected in : {} seconds",
        lsof_time
    );

    let lsof_output = String::from_utf8_lossy(&lsof_output.stdout);

    let mut file = File::create("lsof.txt").expect("Failed to create `lsof.txt`");
    let _ = file
        .write_all(lsof_output.as_bytes())
        .expect("Failed to write to `lsof.txt`");

    let pid_output = Command::new("awk")
        .arg("NR>1 {print $2}")
        .arg("pid.txt")
        .output()
        .expect("Command `awk` failed");

    let pid_output = String::from_utf8_lossy(&pid_output.stdout);
    //FIXME calling `unwrap_or_default()` here will return a 0 if there is an error parsing the string.
    //given that the root pid is 0, this should be fixed.
    let pids: Vec<u32> = pid_output
        .split("\n")
        .map(|x| x.parse::<u32>().ok().unwrap_or_default())
        .collect();

    let port_output = Command::new("awk")
        .arg("NR>1 {print $9}")
        .arg("pid.txt")
        .output()
        .expect("Command `awk` failed");

    let port_output = String::from_utf8_lossy(&port_output.stdout);
    let address_pairs: Vec<&str> = port_output.split("\n").collect();

    // A map of pids to source/destination ports
    let mut port_proc_map: HashMap<u32, Vec<(&str, &str)>> = HashMap::new();
    for (i, pid) in pids.iter().enumerate() {
        let mut address_pair: Vec<&str> = address_pairs[i].split("->").collect();
        if address_pair.len() == 1 {
            address_pair.push("");
        }
        if port_proc_map.contains_key(&pid) {
            port_proc_map
                .get_mut(&pid)
                .unwrap()
                .push((address_pair[0], address_pair[1]))
        } else {
            let mut pairs = Vec::new();
            pairs.push((address_pair[0], address_pair[1]));
            port_proc_map.insert(pid.clone(), pairs);
        }
    }

    print_header();

    let pids: Vec<&u32> = port_proc_map.keys().collect();
    for pid in pids {
        let pairs: Vec<(&str, &str)> = port_proc_map.get(pid).unwrap().to_vec();
        for pair in pairs {
            //FIXME a pid of 0 is invalid in this context; this check is
            // ideally not necessary.
            if *pid != 0 {
                print_stats(*pid, pair.0, pair.1);
            }
        }
    }

    println!();
}

fn print_header() {
    println!();

    println!(
        "{pid:<pid_width$}{source:<width$}{destination:<width$}",
        pid = "PID",
        source = "SOURCE",
        destination = "DESTINATION",
        pid_width = 20,
        width = 50
    );
}

fn print_stats(pid: u32, source: &str, destination: &str) {
    println!(
        "{pid:<pid_width$}{source:<width$}{destination:<width$}",
        pid = pid,
        source = source,
        destination = destination,
        pid_width = 20,
        width = 50
    );
}

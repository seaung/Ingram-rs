use crate::config::Config;
use crate::data::{Data, SnapshotPipeline};
use crate::fingerprint::fingerprint;
use crate::net::{parse_target, port_scan};
use crate::poc::PocResult;
use crate::pocs::get_poc_dict;
use anyhow::Result;
use log::{error, info};
use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Core {
    config: Arc<Config>,
    data: Arc<Data>,
    snapshot_pipeline: Option<SnapshotPipeline>,
    poc_dict: Arc<HashMap<String, Vec<Arc<dyn crate::poc::Poc>>>>,
}

impl Core {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let data = Arc::new(Data::new(config.clone())?);
        let poc_dict = Arc::new(get_poc_dict(&config));

        let snapshot_pipeline = if !config.disable_snapshot {
            Some(SnapshotPipeline::new(config.clone(), config.th_num))
        } else {
            None
        };

        Ok(Core {
            config,
            data,
            snapshot_pipeline,
            poc_dict,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Running at {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));
        info!("Config: {:?}", self.config);

        let status_handle = self.start_status_bar();

        let scan_handles = self.start_scan();

        for handle in scan_handles {
            let _ = handle.join();
        }

        if let Some(ref pipeline) = self.snapshot_pipeline {
            while !pipeline.is_empty() {
                thread::sleep(Duration::from_millis(100));
            }
        }

        let _ = status_handle.join();

        self.report();

        Ok(())
    }

    fn start_status_bar(&self) -> JoinHandle<()> {
        let total = self.data.get_total();
        let done = self.data.get_done_arc();
        let found = self.data.get_found_arc();
        let snapshot_pipeline: Option<(Arc<std::sync::atomic::AtomicUsize>, Arc<std::sync::atomic::AtomicUsize>)> = self.snapshot_pipeline.as_ref().map(|p| {
            (
                p.get_task_count(),
                p.get_snapshot_done(),
            )
        });

        thread::spawn(move || {
            let progress = indicatif::ProgressBar::new(total as u64);
            progress.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                    .unwrap()
                    .progress_chars("=>-"),
            );

            loop {
                let current_done = done.load(std::sync::atomic::Ordering::SeqCst);
                let current_found = found.load(std::sync::atomic::Ordering::SeqCst);

                progress.set_position(current_done as u64);
                progress.set_message(format!("Found: {}", current_found));

                if let Some((task_count, snapshot_done)) = &snapshot_pipeline {
                    let pending = task_count.load(std::sync::atomic::Ordering::SeqCst);
                    let snap_done = snapshot_done.load(std::sync::atomic::Ordering::SeqCst);
                    progress.set_message(format!(
                        "Found: {} | Snapshots: {} (pending: {})",
                        current_found, snap_done, pending
                    ));
                }

                if current_done >= total {
                    progress.finish();
                    break;
                }

                thread::sleep(Duration::from_millis(100));
            }
        })
    }

    fn start_scan(&self) -> Vec<JoinHandle<()>> {
        let mut handles = Vec::new();
        let ip_generator = self.data.ip_generator();
        let ip_list: Vec<String> = ip_generator.collect();

        let chunk_size = (ip_list.len() / self.config.th_num).max(1);

        for chunk in ip_list.chunks(chunk_size) {
            let chunk_vec = chunk.to_vec();
            let config = Arc::clone(&self.config);
            let poc_dict = Arc::clone(&self.poc_dict);
            let data_clone = Arc::clone(&self.data);

            let snapshot_pipeline: Option<(Arc<crossbeam::channel::Sender<(Arc<dyn Fn(&PocResult, &Config) -> Result<usize> + Send + Sync>, PocResult)>>, Arc<std::sync::atomic::AtomicUsize>)> = self.snapshot_pipeline.as_ref().map(|p| {
                (
                    p.get_sender(),
                    p.get_task_count(),
                )
            });

            let handle = thread::spawn(move || {
                for target in chunk_vec {
                    Self::scan_target(
                        &target,
                        &config,
                        &poc_dict,
                        &data_clone,
                        snapshot_pipeline.as_ref(),
                    );
                }
            });

            handles.push(handle);
        }

        handles
    }

    fn scan_target(
        target: &str,
        config: &Arc<Config>,
        poc_dict: &Arc<HashMap<String, Vec<Arc<dyn crate::poc::Poc>>>>,
        data: &Data,
        snapshot_pipeline: Option<&(Arc<crossbeam::channel::Sender<(Arc<dyn Fn(&PocResult, &Config) -> Result<usize> + Send + Sync>, PocResult)>>, Arc<std::sync::atomic::AtomicUsize>)>,
    ) {
        let (ip, specified_port) = match parse_target(target) {
            Some(result) => result,
            None => return,
        };

        let ports = if let Some(port) = specified_port {
            vec![port]
        } else {
            config.ports.clone()
        };

        for port in ports {
            if port_scan(&ip, port, config.timeout) {
                info!("{} port {} is open", ip, port);

                if let Some(product) = fingerprint(&ip, port, config) {
                    info!("{}:{} is {}", ip, port, product);

                    let mut verified = false;

                    if let Some(pocs) = poc_dict.get(&product) {
                        for poc in pocs {
                            if let Some(result) = poc.verify(&ip, port, config) {
                                verified = true;
                                data.add_found();
                                data.add_vulnerable(&result);

                                if !config.disable_snapshot {
                                    if let Some((sender, _)) = snapshot_pipeline {
                                        let poc_arc = Arc::clone(poc);
                                        let exploit_func = crate::poc::to_exploit_func(poc_arc);
                                        let _ = sender.send((exploit_func, result));
                                    }
                                }
                            }
                        }
                    }

                    if !verified {
                        data.add_not_vulnerable(&ip, port, &product);
                    }
                }
            }
        }

        data.add_done();
    }

    fn report(&self) {
        let results_file = std::path::Path::new(&self.config.out_dir)
            .join(&self.config.vulnerable);

        if !results_file.exists() {
            return;
        }

        let mut results: std::collections::HashMap<String, std::collections::HashMap<String, usize>> =
            std::collections::HashMap::new();
        let mut total_count = 0;
        let mut max_count = 0;

        if let Ok(file) = std::fs::File::open(&results_file) {
            let reader = std::io::BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 6 {
                        let product = parts[2];
                        let vul_name = parts[5];

                        *results
                            .entry(product.to_string())
                            .or_insert_with(std::collections::HashMap::new)
                            .entry(vul_name.to_string())
                            .or_insert(0) += 1;

                        total_count += 1;
                    }
                }
            }
        }

        for vuls in results.values() {
            for count in vuls.values() {
                max_count = max_count.max(*count);
            }
        }

        if total_count > 0 {
            println!("\n");
            println!("{}", "-".repeat(46));
            println!("{:^20}{}", "", "REPORT");
            println!("{}", "-".repeat(46));

            for (product, vuls) in &results {
                let dev_sum: usize = vuls.values().sum();
                println!("{}", colored::Colorize::bright_red(format!("{} {}", product, dev_sum).as_str()));

                let mut vul_vec: Vec<(&String, &usize)> = vuls.iter().collect();
                vul_vec.sort_by(|a, b| b.1.cmp(a.1));

                for (vul_name, vul_count) in vul_vec {
                    let block_num = (*vul_count as f64 / max_count as f64 * 25.0) as usize;
                    let bar = "▥".repeat(block_num);
                    println!(
                        "{:>18} | {} {}",
                        colored::Colorize::green(vul_name.as_str()),
                        bar,
                        vul_count
                    );
                }
            }

            println!(
                "{}",
                colored::Colorize::bright_yellow(format!(
                    "{:>46}",
                    format!("sum: {}", total_count)
                ).as_str())
            );
            println!("{}", "-".repeat(46));
            println!("\n");
        }
    }
}

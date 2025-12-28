use crate::config::Config;
use crate::net::{get_all_ip, get_ip_seg_len, parse_target};
use crate::poc::PocResult;
use anyhow::{Context, Result};
use chrono::Utc;
use crossbeam::channel::{bounded, Receiver, Sender};
use log::error;
use md5::compute;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

pub struct Data {
    config: Arc<Config>,
    total: Arc<AtomicUsize>,
    done: Arc<AtomicUsize>,
    found: Arc<AtomicUsize>,
    vulnerable_file: File,
    not_vulnerable_file: File,
    task_id: String,
    create_time: i64,
}

impl Data {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let out_dir = Path::new(&config.out_dir);
        if !out_dir.exists() {
            std::fs::create_dir_all(out_dir)?;
        }

        let snapshots_dir = out_dir.join(&config.snapshots);
        if !snapshots_dir.exists() {
            std::fs::create_dir_all(&snapshots_dir)?;
        }

        let vulnerable_path = out_dir.join(&config.vulnerable);
        let not_vulnerable_path = out_dir.join(&config.not_vulnerable);

        let vulnerable_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&vulnerable_path)
            .context("Failed to open vulnerable file")?;

        let not_vulnerable_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&not_vulnerable_path)
            .context("Failed to open not_vulnerable file")?;

        let task_id = format!(
            "{:x}",
            compute(format!("{}{}", config.in_file, config.out_dir).as_bytes())
        );
        let create_time = Utc::now().timestamp();

        let data = Data {
            config,
            total: Arc::new(AtomicUsize::new(0)),
            done: Arc::new(AtomicUsize::new(0)),
            found: Arc::new(AtomicUsize::new(0)),
            vulnerable_file,
            not_vulnerable_file,
            task_id,
            create_time,
        };

        data.load_state()?;
        data.calculate_total()?;

        Ok(data)
    }

    fn load_state(&self) -> Result<()> {
        let state_file = Path::new(&self.config.out_dir).join(format!(".{}", self.task_id));
        if state_file.exists() {
            let file = File::open(&state_file)?;
            let reader = BufReader::new(file);
            if let Some(Ok(line)) = reader.lines().next() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 3 {
                    if let Ok(done) = parts[0].parse::<usize>() {
                        self.done.store(done, Ordering::SeqCst);
                    }
                    if let Ok(found) = parts[1].parse::<usize>() {
                        self.found.store(found, Ordering::SeqCst);
                    }
                    if let Ok(_runned_time) = parts[2].parse::<f64>() {
                    }
                }
            }
        }
        Ok(())
    }

    fn calculate_total(&self) -> Result<()> {
        let file = File::open(&self.config.in_file)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let strip_line = line.trim();
                if !strip_line.is_empty() && !strip_line.starts_with('#') {
                    let count = get_ip_seg_len(strip_line);
                    self.total.fetch_add(count, Ordering::SeqCst);
                }
            }
        }

        Ok(())
    }

    pub fn ip_generator(&self) -> impl Iterator<Item = String> + '_ {
        let file_path = self.config.in_file.clone();
        let skip_count = self.done.load(Ordering::SeqCst);

        let file = File::open(&file_path).unwrap();
        let reader = BufReader::new(file);

        reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .flat_map(move |target| get_all_ip(&target))
            .skip(skip_count)
    }

    pub fn add_done(&self) {
        self.done.fetch_add(1, Ordering::SeqCst);
        self.record_state();
    }

    pub fn add_found(&self) {
        self.found.fetch_add(1, Ordering::SeqCst);
    }

    pub fn add_vulnerable(&self, result: &PocResult) {
        let line = format!(
            "{},{},{},{},{},{}\n",
            result.ip,
            result.port,
            result.product,
            result.user,
            result.password,
            result.vul_name
        );

        let mut file = &self.vulnerable_file;
        if let Err(e) = writeln!(file, "{}", line.trim()) {
            error!("Failed to write vulnerable result: {}", e);
        }
        let _ = file.flush();
    }

    pub fn add_not_vulnerable(&self, ip: &str, port: u16, product: &str) {
        let mut file = &self.not_vulnerable_file;
        if let Err(e) = writeln!(file, "{},{},{}", ip, port, product) {
            error!("Failed to write not_vulnerable result: {}", e);
        }
        let _ = file.flush();
    }

    fn record_state(&self) {
        let done = self.done.load(Ordering::SeqCst);
        let found = self.found.load(Ordering::SeqCst);
        let runned_time = Utc::now().timestamp() - self.create_time;

        if done % 20 == 0 {
            let state_file = Path::new(&self.config.out_dir).join(format!(".{}", self.task_id));
            if let Ok(mut file) = File::create(&state_file) {
                let _ = writeln!(file, "{},{},{}", done, found, runned_time);
            }
        }
    }

    pub fn get_total(&self) -> usize {
        self.total.load(Ordering::SeqCst)
    }

    pub fn get_done(&self) -> usize {
        self.done.load(Ordering::SeqCst)
    }

    pub fn get_found(&self) -> usize {
        self.found.load(Ordering::SeqCst)
    }

    pub fn get_done_arc(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.done)
    }

    pub fn get_found_arc(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.found)
    }

    pub fn is_finished(&self) -> bool {
        self.done.load(Ordering::SeqCst) >= self.total.load(Ordering::SeqCst)
    }
}

pub struct SnapshotPipeline {
    config: Arc<Config>,
    sender: Arc<Sender<(Arc<dyn Fn(&PocResult, &Config) -> Result<usize> + Send + Sync>, PocResult)>>,
    receiver: Receiver<(Arc<dyn Fn(&PocResult, &Config) -> Result<usize> + Send + Sync>, PocResult)>,
    task_count: Arc<AtomicUsize>,
    done: Arc<AtomicUsize>,
    _handles: Vec<JoinHandle<()>>,
}

impl SnapshotPipeline {
    pub fn new(config: Arc<Config>, num_workers: usize) -> Self {
        let (sender, receiver) = bounded(num_workers * 2);
        let sender = Arc::new(sender);
        let task_count = Arc::new(AtomicUsize::new(0));
        let done = Arc::new(AtomicUsize::new(0));

        let snapshots_dir = PathBuf::from(&config.out_dir).join(&config.snapshots);
        if let Ok(entries) = std::fs::read_dir(&snapshots_dir) {
            let count = entries.filter_map(|e| e.ok()).count();
            done.store(count, Ordering::SeqCst);
        }

        let mut handles = Vec::new();
        for _ in 0..num_workers {
            let receiver_clone: Receiver<(Arc<dyn Fn(&PocResult, &Config) -> Result<usize> + Send + Sync>, PocResult)> = receiver.clone();
            let task_count_clone = task_count.clone();
            let done_clone = done.clone();
            let config_clone = config.clone();

            let handle = thread::spawn(move || loop {
                match receiver_clone.recv() {
                    Ok((exploit_func, result)) => {
                        if let Ok(count) = exploit_func(&result, &config_clone) {
                            done_clone.fetch_add(count, Ordering::SeqCst);
                        }
                        task_count_clone.fetch_sub(1, Ordering::SeqCst);
                    }
                    Err(_) => break,
                }
            });

            handles.push(handle);
        }

        SnapshotPipeline {
            config,
            sender,
            receiver,
            task_count,
            done,
            _handles: handles,
        }
    }

    pub fn put(&self, exploit_func: Arc<dyn Fn(&PocResult, &Config) -> Result<usize> + Send + Sync>, result: PocResult) {
        self.task_count.fetch_add(1, Ordering::SeqCst);
        let _ = self.sender.send((exploit_func, result));
    }

    pub fn get_done(&self) -> usize {
        self.done.load(Ordering::SeqCst)
    }

    pub fn is_empty(&self) -> bool {
        self.task_count.load(Ordering::SeqCst) == 0
    }

    pub fn get_sender(&self) -> Arc<Sender<(Arc<dyn Fn(&PocResult, &Config) -> Result<usize> + Send + Sync>, PocResult)>> {
        Arc::clone(&self.sender)
    }

    pub fn get_task_count(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.task_count)
    }

    pub fn get_snapshot_done(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.done)
    }
}

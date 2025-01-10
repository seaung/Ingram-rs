use std::path::PathBuf;
use clap::{Arg, Command};

#[derive(Debug)]
pub struct Cli {
    input_file: PathBuf,
    output_file: PathBuf,
    ports: Option<Vec<u16>>,
    thread_nums: usize,
    timeout: u64,
    disable_snapshot: bool,
    debug: bool,
}

pub fn get_args() -> Cli {
    let matches = Command::new("网络摄像头漏洞扫描")
        .version("0.1.0")
        .author("Github: https://wwww.github.com/seaung")
        .about("一个用于网络摄像头漏洞扫描审计命令行程序.")
        .arg(
            Arg::new("input_file")
                .short('i')
                .long("input_file")
                .value_name("FILE")
                .help("目标列表文件")
                .required(true)
        )
        .arg(
            Arg::new("output_file")
                .short('o')
                .long("output_file")
                .value_name("DIR")
                .help("扫描结果输出路径")
                .required(true)
        )
        .arg(
            Arg::new("ports")
                .short('p')
                .long("ports")
                .value_name("PORTS")
                .help("目标端口")
                .required(true)
        )
        .arg(
            Arg::new("thread_nums")
                .short('T')
                .long("thread_nums")
                .help("扫描线程数")
                .required(true)
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .help("设置扫描超时时间")
                .required(true)
        )
        .arg(
            Arg::new("disable_snapshot")
                .short('s')
                .long("disable_snapshot")
                .help("是否存储扫描快照")
                .required(true)
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("是否打印调试日志")
                .required(true)
        ).get_matches();

    Cli{
        input_file: PathBuf::from(matches.get_one::<String>("input_file").expect("required")),
        output_file: PathBuf::from(matches.get_one::<String>("output_file").expect("required")),
        ports: matches.get_many::<String>("ports").map(|v| {
            v.into_iter()
                .filter_map(|val| val.parse().ok()).collect()
        }),
        thread_nums: *matches.get_one::<String>("thread_nums").unwrap_or(&"150".to_string()).parse().unwrap(),
        timeout: *matches.get_one::<String>("timeout").unwrap_or(&"3".to_string()).parse().unwrap(),
        disable_snapshot: *matches.get_one::<bool>("disable_snapshot").unwrap_or(&false),
        debug: *matches.get_one::<bool>("debug").unwrap_or(&true),
    }
}
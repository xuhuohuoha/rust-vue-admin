use serde::Serialize;

/// 获取系统信息
#[derive(Debug, Serialize, Default, Clone)]
pub struct SysInfo {
    /// 服务器信息
    pub server: Server,
    /// CPU信息
    pub cpu: Cpu,
    /// CPU负载信息
    pub cpu_load: CpuLoad,
    /// 内存信息
    pub memory: Memory,
    /// 磁盘信息
    pub process: Process,
    /// 网络信息
    pub network: Vec<Network>,
}

/// 获取CPU信息
#[derive(Debug, Serialize, Default, Clone)]
pub struct Cpu {
    /// CPU名称
    pub name: String,
    /// CPU架构
    pub arch: String,
    /// CPU型号
    pub processors: usize,
    /// CPU频率
    pub frequency: u64,
    /// CPU核心数
    pub cores: String,
    /// CPU逻辑核心数
    pub total_use: f32,
}

/// 获取CPU负载信息
#[derive(Debug, Serialize, Default, Clone)]
pub struct CpuLoad {
    /// 最近一分钟内的CPU平均负载
    pub one: f64,
    /// 最近五分钟内的CPU平均负载
    pub five: f64,
    /// 最近十五分钟内的CPU平均负载
    pub fifteen: f64,
}

/// 获取内存信息
#[derive(Debug, Serialize, Default, Clone)]
pub struct Memory {
    /// 总内存
    pub total_memory: u64,
    /// 已使用内存
    pub used_memory: u64,
    /// 总交换内存
    pub total_swap: u64,
    /// 已使用交换内存
    pub used_swap: u64,
}

/// 获取服务器信息
#[derive(Debug, Serialize, Default, Clone)]
pub struct Server {
    /// 操作系统
    pub os_name: String,
    /// 主机名
    pub host_name: String,
    /// 系统版本
    pub system_version: String,
    /// 系统内核
    pub system_kernel: String,
}

/// 获取进程信息
#[derive(Debug, Serialize, Default, Clone)]
pub struct Process {
    /// 进程名称
    pub name: String,
    /// 已使用物理内存
    pub used_memory: u64,
    /// 已使用虚拟内存
    pub used_virtual_memory: u64,
    /// CPU占用率
    pub cup_usage: f32,
    /// 启动时间
    pub start_time: u64,
    /// 运行时间
    pub run_time: u64,
    /// 磁盘使用情况
    pub disk_usage: DiskUsage,
}

/// 获取磁盘信息
#[derive(Debug, Serialize, Default, Clone)]
pub struct DiskUsage {
    /// 自上次统计以来读取的字节数
    pub read_bytes: u64,
    /// 总共读取的字节数
    pub total_read_bytes: u64,
    /// 自上次统计以来写入的字节数
    pub written_bytes: u64,
    /// 总共写入的字节数
    pub total_written_bytes: u64,
}

/// 获取网络信息
#[derive(Debug, Serialize, Default, Clone)]
pub struct Network {
    /// 网络接口名称
    pub name: String,
    /// 接收数据量
    pub received: u64,
    /// 总接收数据量
    pub total_received: u64,
    /// 发送数据量
    pub transmitted: u64,
    /// 总发送数据量
    pub total_transmitted: u64,
}

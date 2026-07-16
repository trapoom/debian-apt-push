use std::fs;
use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub model_name: String,
    pub cores: usize,
    pub max_freq_mhz: f64,
    pub current_freq_mhz: f64,
}

#[derive(Debug, Clone)]
pub struct CpuStats {
    pub usage_percent: f64,
    pub temp_celsius: f64,
    #[allow(dead_code)]
    pub per_core_usage: Vec<f64>,
    #[allow(dead_code)]
    pub per_core_temp: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_gb: f64,
    pub used_gb: f64,
    #[allow(dead_code)]
    pub available_gb: f64,
    pub usage_percent: f64,
    #[allow(dead_code)]
    pub buffers_gb: f64,
    #[allow(dead_code)]
    pub cached_gb: f64,
}

#[derive(Debug, Clone)]
pub struct BatteryStats {
    pub capacity_percent: u8,
    pub status: String,
    #[allow(dead_code)]
    pub health: String,
    #[allow(dead_code)]
    pub technology: String,
    pub voltage_mv: u32,
}

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub battery: Option<BatteryStats>,
    pub uptime_secs: u64,
}

pub struct SystemMonitor {
    battery_path: Option<String>,
    prev_cpu_stat: Option<(u64, u64)>,
}

impl SystemMonitor {
    pub fn new(battery_path: Option<String>) -> Self {
        SystemMonitor {
            battery_path,
            prev_cpu_stat: None,
        }
    }

    #[allow(dead_code)]
    pub fn get_cpu_info(&self) -> Result<CpuInfo> {
        let cpuinfo = fs::read_to_string("/proc/cpuinfo")?;
        let mut model_name = String::from("Unknown");
        let mut cores = 0;
        let mut max_freq = 2400.0;

        for line in cpuinfo.lines() {
            if line.starts_with("model name") {
                if let Some(val) = line.split(':').nth(1) {
                    model_name = val.trim().to_string();
                }
            }
            if line.starts_with("processor") {
                cores += 1;
            }
            if line.starts_with("max freq") {
                if let Some(freq) = line.split(':').nth(1).and_then(|s| s.trim().parse::<f64>().ok()) {
                    max_freq = freq;
                }
            }
        }

        let current_freq = self.get_avg_cpu_freq().unwrap_or(2400.0);

        Ok(CpuInfo {
            model_name,
            cores: cores.max(1),
            max_freq_mhz: max_freq,
            current_freq_mhz: current_freq,
        })
    }

    pub fn get_cpu_stats(&mut self) -> Result<CpuStats> {
        let cpu_usage = self.calculate_cpu_usage()?;
        let temp = self.get_cpu_temperature()?;
        let per_core_usage = self.get_per_core_usage()?;
        let per_core_temp = self.get_per_core_temps()?;

        Ok(CpuStats {
            usage_percent: cpu_usage,
            temp_celsius: temp,
            per_core_usage,
            per_core_temp,
        })
    }

    pub fn get_memory_stats(&self) -> Result<MemoryStats> {
        let meminfo = fs::read_to_string("/proc/meminfo")?;
        let mut mem_total: u64 = 0;
        let mut mem_available: u64 = 0;
        let mut mem_buffers: u64 = 0;
        let mut mem_cached: u64 = 0;

        for line in meminfo.lines() {
            if let Some(value) = self.extract_kb_value(line, "MemTotal") {
                mem_total = value;
            } else if let Some(value) = self.extract_kb_value(line, "MemAvailable") {
                mem_available = value;
            } else if let Some(value) = self.extract_kb_value(line, "Buffers") {
                mem_buffers = value;
            } else if let Some(value) = self.extract_kb_value(line, "Cached") {
                mem_cached = value;
            }
        }

        let mem_used = mem_total.saturating_sub(mem_available);
        let total_gb = mem_total as f64 / 1024.0 / 1024.0;
        let used_gb = mem_used as f64 / 1024.0 / 1024.0;
        let available_gb = mem_available as f64 / 1024.0 / 1024.0;
        let buffers_gb = mem_buffers as f64 / 1024.0 / 1024.0;
        let cached_gb = mem_cached as f64 / 1024.0 / 1024.0;

        let usage_percent = if mem_total > 0 {
            (mem_used as f64 / mem_total as f64) * 100.0
        } else {
            0.0
        };

        Ok(MemoryStats {
            total_gb,
            used_gb,
            available_gb,
            usage_percent,
            buffers_gb,
            cached_gb,
        })
    }

    pub fn get_battery_stats(&self) -> Result<Option<BatteryStats>> {
        let path = match &self.battery_path {
            Some(p) => p,
            None => return Ok(None),
        };

        let parent = std::path::Path::new(path)
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid battery path"))?;

        let capacity = fs::read_to_string(parent.join("capacity"))
            .ok()
            .and_then(|s| s.trim().parse::<u8>().ok())
            .unwrap_or(0);

        let status = fs::read_to_string(parent.join("status"))
            .ok()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let health = fs::read_to_string(parent.join("health"))
            .ok()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let technology = fs::read_to_string(parent.join("technology"))
            .ok()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let voltage_mv = fs::read_to_string(parent.join("voltage_now"))
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
            .unwrap_or(0) / 1000;

        Ok(Some(BatteryStats {
            capacity_percent: capacity,
            status,
            health,
            technology,
            voltage_mv,
        }))
    }

    pub fn get_uptime(&self) -> Result<u64> {
        let uptime_str = fs::read_to_string("/proc/uptime")?;
        let secs = uptime_str
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) as u64;
        Ok(secs)
    }

    pub fn get_system_stats(&mut self) -> Result<SystemStats> {
        let cpu = self.get_cpu_stats()?;
        let memory = self.get_memory_stats()?;
        let battery = self.get_battery_stats()?;
        let uptime_secs = self.get_uptime()?;

        Ok(SystemStats {
            cpu,
            memory,
            battery,
            uptime_secs,
        })
    }

    fn calculate_cpu_usage(&mut self) -> Result<f64> {
        let stat_str = fs::read_to_string("/proc/stat")?;
        let first_line = stat_str.lines().next().ok_or_else(|| anyhow::anyhow!("Empty /proc/stat"))?;

        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 5 {
            return Ok(0.0);
        }

        let user: u64 = parts[1].parse().unwrap_or(0);
        let nice: u64 = parts[2].parse().unwrap_or(0);
        let system: u64 = parts[3].parse().unwrap_or(0);
        let idle: u64 = parts[4].parse().unwrap_or(0);

        let work = user + nice + system;
        let total = work + idle;

        if let Some((prev_work, prev_total)) = self.prev_cpu_stat {
            let work_delta = work.saturating_sub(prev_work);
            let total_delta = total.saturating_sub(prev_total);

            let usage = if total_delta > 0 {
                (work_delta as f64 / total_delta as f64) * 100.0
            } else {
                0.0
            };

            self.prev_cpu_stat = Some((work, total));
            Ok(usage.min(100.0))
        } else {
            self.prev_cpu_stat = Some((work, total));
            Ok(0.0)
        }
    }

    fn get_cpu_temperature(&self) -> Result<f64> {
        const THERMAL_ZONE: &str = "/sys/class/thermal/thermal_zone0/temp";
        
        if let Ok(content) = fs::read_to_string(THERMAL_ZONE) {
            if let Ok(millidegrees) = content.trim().parse::<f64>() {
                return Ok(millidegrees / 1000.0);
            }
        }

        if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(content) = fs::read_to_string(path.join("temp1_input")) {
                    if let Ok(millidegrees) = content.trim().parse::<f64>() {
                        return Ok(millidegrees / 1000.0);
                    }
                }
            }
        }

        Ok(0.0)
    }

    fn get_per_core_usage(&self) -> Result<Vec<f64>> {
        let stat_str = fs::read_to_string("/proc/stat")?;
        let mut usage_vec = Vec::new();

        for line in stat_str.lines().skip(1) {
            if !line.starts_with("cpu") {
                break;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 5 {
                continue;
            }

            let user: u64 = parts[1].parse().unwrap_or(0);
            let nice: u64 = parts[2].parse().unwrap_or(0);
            let system: u64 = parts[3].parse().unwrap_or(0);
            let idle: u64 = parts[4].parse().unwrap_or(0);

            let total = user + nice + system + idle;
            let work = user + nice + system;

            let usage = if total > 0 {
                (work as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            usage_vec.push(usage.min(100.0));
        }

        Ok(usage_vec)
    }

    fn get_per_core_temps(&self) -> Result<Vec<f64>> {
        let mut temps = Vec::new();

        if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
            for entry in entries.flatten() {
                let path = entry.path();
                for i in 1..=16 {
                    let temp_file = path.join(format!("temp{}_input", i));
                    if let Ok(content) = fs::read_to_string(&temp_file) {
                        if let Ok(millidegrees) = content.trim().parse::<f64>() {
                            temps.push(millidegrees / 1000.0);
                        }
                    }
                }
                if !temps.is_empty() {
                    break;
                }
            }
        }

        Ok(temps)
    }

    #[allow(dead_code)]
    fn get_avg_cpu_freq(&self) -> Result<f64> {
        let mut total_freq = 0.0;
        let mut count = 0;

        for i in 0..16 {
            let freq_file = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", i);
            if let Ok(content) = fs::read_to_string(&freq_file) {
                if let Ok(khz) = content.trim().parse::<f64>() {
                    total_freq += khz / 1000.0;
                    count += 1;
                }
            }
        }

        if count > 0 {
            Ok(total_freq / count as f64)
        } else {
            Ok(2400.0)
        }
    }

    fn extract_kb_value(&self, line: &str, key: &str) -> Option<u64> {
        if !line.starts_with(key) {
            return None;
        }
        line.split_whitespace().nth(1)?.parse().ok()
    }
}

pub fn detect_battery_path() -> Option<String> {
    if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("BAT") {
                let capacity_path = entry.path().join("capacity");
                if capacity_path.exists() {
                    return Some(capacity_path.to_string_lossy().to_string());
                }
            }
        }
    }
    None
}
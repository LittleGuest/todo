/// 雪花算法
use std::sync::atomic::{AtomicI64, Ordering};

pub struct Snowflake {
    // 开始时间戳
    epoch: i64,
    // 数据中心ID
    datacenter_id: i64,
    // 工作机器ID
    worker_id: i64,
    // 序列号
    sequence: AtomicI64,
    // 上次产生ID的时间戳
    last_timestamp: AtomicI64,
}

impl Default for Snowflake {
    fn default() -> Self {
        Self {
            epoch: 1618710339362,
            datacenter_id: 0,
            worker_id: 0,
            sequence: AtomicI64::new(0),
            last_timestamp: AtomicI64::new(-1),
        }
    }
}

impl Snowflake {
    pub fn new(datacenter_id: i64, worker_id: i64) -> Self {
        Self {
            epoch: 1618710339362,
            datacenter_id,
            worker_id,
            sequence: AtomicI64::new(0),
            last_timestamp: AtomicI64::new(-1),
        }
    }

    pub fn next_id(&self) -> i64 {
        // 获取当前时间戳
        let mut timestamp = self.time_gen();
        let last_timestamp = self.last_timestamp.fetch_or(0, Ordering::Relaxed);
        let sequence = self.sequence.fetch_or(0, Ordering::Relaxed);

        // 判断当前时间戳是否小于上次产生ID的时间戳
        if timestamp < last_timestamp {
            panic!("时间后退，拒绝产生ID");
        }

        // 判断当前时间戳是否等于上次产生ID的时间戳
        if timestamp == last_timestamp {
            let sequence = (sequence + 1) & -1 ^ (-1 << 12);
            self.sequence.swap(sequence, Ordering::Relaxed);
            // 判断序列号是否溢出
            if sequence == 0 {
                timestamp = self.til_next_millis();
            }
        } else {
            self.sequence.swap(0, Ordering::Relaxed);
        }

        self.last_timestamp.swap(timestamp, Ordering::Relaxed);

        ((timestamp - self.epoch) << 22)
            | (self.datacenter_id << 17)
            | (self.worker_id << 12)
            | self.sequence.fetch_or(0, Ordering::Relaxed)
    }

    // 获取当前毫秒时间戳
    fn time_gen(&self) -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64
    }

    // 阻塞获取下一毫秒时间戳
    fn til_next_millis(&self) -> i64 {
        let mut timestamp = self.time_gen();
        let last_timestamp = self.last_timestamp.fetch_or(0, Ordering::Relaxed);

        while timestamp < last_timestamp {
            timestamp = self.time_gen();
        }
        timestamp
    }
}

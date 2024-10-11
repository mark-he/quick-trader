
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use chrono::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct KLine {
    pub symbol: String,
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i32,
    pub turnover: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct KLineWrapper {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    data: KLine,
}

pub struct KLineCombiner {
    starting_hour: Option<u32>,
    period: u32,
    unit: char,
    current_k_line: Option<KLineWrapper>,
    k_lines: LimitedQueue<KLine>,
}


struct LimitedQueue<T> {
    queue: VecDeque<T>,
    limit: u32,
}

impl <T> LimitedQueue<T> {
    fn new(limit: u32) -> Self {
        Self {
            queue: VecDeque::new(),
            limit,
        }
    }

    fn enqueue(&mut self, value: T) {
        if self.queue.len() == self.limit as usize{
            self.queue.pop_front();
        }
        self.queue.push_back(value);
    }
}

impl KLineCombiner {
    pub fn new(period_str: &str, count: u32, starting_hour: Option<u32>) -> Self {
        let (period, unit) = Self::parse_period(period_str);
        Self {
            starting_hour,
            period,
            unit,
            current_k_line: None,
            k_lines: LimitedQueue::new(count),
        }
    }

    pub fn init(&mut self, k_lines: Vec<KLine>) {
        for k_line in k_lines {
            self.combine_tick(&k_line, false);
        }
    }

    fn parse_period(period_str: &str) -> (u32, char) {
        let period: u32 = period_str[0..period_str.len() - 1].parse().unwrap();
        let unit = period_str.chars().last().unwrap();
        (period, unit)
    }

    fn calculate_time(tick_time: DateTime<Utc>, period: u32, unit: char, starting_hour: Option<u32>) -> (DateTime<Utc>, DateTime<Utc>) {
        match unit {
            'm' => {
                let start_minute = tick_time.minute();
                let start_time: DateTime<Utc> = Utc.with_ymd_and_hms(tick_time.year(), tick_time.month(), tick_time.day(),tick_time.hour(), start_minute, 0).unwrap();
                let end_time = start_time + Duration::minutes(period as i64);
                if let Some(s) = starting_hour {
                    if start_time.hour() < s && end_time.hour() >= s {
                        end_time.with_hour(s);
                    }
                }
                (start_time, end_time)
            }
            'h' => {
                let start_hour: u32 = tick_time.hour();
                let start_time: DateTime<Utc> = Utc.with_ymd_and_hms(tick_time.year(), tick_time.month(), tick_time.day(),start_hour, 0, 0).unwrap();
                let end_time = start_time + Duration::hours(period as i64);
                if let Some(s) = starting_hour {
                    if start_time.hour() < s && end_time.hour() >= s {
                        end_time.with_hour(s);
                    }
                }
                (start_time, end_time)
            }
            'd' => {
                let start_day = tick_time.day();
                let mut start_time: DateTime<Utc> = Utc.with_ymd_and_hms(tick_time.year(), tick_time.month(), start_day, 0, 0, 0).unwrap();
                if let Some(s) = starting_hour {
                    if tick_time.hour() < s {
                        start_time = start_time - Duration::days(1) + Duration::hours(s as i64);
                    }
                }
                let end_time = start_time + Duration::days(period as i64);
                (start_time, end_time)
            }
            'w' => {
                let start_week = tick_time.iso_week().week();
                let start_day = Utc.with_ymd_and_hms(tick_time.year(), 1, 1, 0, 0, 0).unwrap() + Duration::weeks(start_week as i64);
                let mut start_time: DateTime<Utc> = Utc.with_ymd_and_hms(start_day.year(), start_day.month(), start_day.day(),0, 0, 0).unwrap();
                if let Some(s) = starting_hour {
                    if tick_time.hour() < s {
                        start_time = start_time - Duration::days(1) + Duration::hours(s as i64);
                    }
                }
                let end_time = start_time + Duration::weeks(period as i64);
                (start_time, end_time)
            }
            _ => panic!("Invalid time unit"),
        }
    }

    pub fn combine_tick(&mut self, tick: &KLine, return_new: bool) -> Option<KLine> {
        let tick_time: DateTime<Utc> = DateTime::from_naive_utc_and_offset(NaiveDateTime::parse_from_str(&tick.datetime, "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
        if self.current_k_line.is_none() || tick_time >= self.current_k_line.as_ref().unwrap().end_time {
            let (start_time, end_time) = Self::calculate_time(tick_time, self.period, self.unit, self.starting_hour);
            let mut ret = None;
            if let Some(k_line) = self.current_k_line.take() {
                if return_new {
                    ret = Some(k_line.data.clone());
                }
                self.k_lines.enqueue(k_line.data);
            }
            let mut k_line = KLineWrapper {
                start_time,
                end_time,
                data: tick.clone(),
            };
            k_line.data.datetime = start_time.format("%Y-%m-%d %H:%M:%S").to_string();
            self.current_k_line = Some(k_line);
            ret
        } else {
            let k_line = self.current_k_line.as_mut().unwrap();
            if tick.high > k_line.data.high {
                k_line.data.high = tick.high;
            }
            if tick.low < k_line.data.low {
                k_line.data.low = tick.low;
            }
            k_line.data.volume += tick.volume;
            k_line.data.turnover += tick.turnover;
            k_line.data.close = tick.close;
            None
        }
    }

    pub fn get_k_lines(&self) -> &VecDeque<KLine> {
        &self.k_lines.queue
    }

    pub fn close(&mut self, return_new: bool) -> Option<KLine> {
        let mut ret = None;
        if let Some(k_line) = self.current_k_line.take() {
            if return_new {
                ret = Some(k_line.data.clone());
            }
            self.k_lines.enqueue(k_line.data);
        }
        ret
    }
}
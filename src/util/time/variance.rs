use std::collections::VecDeque;
use std::time::{Duration,SystemTime};


pub struct ProfArgs {
    pub label: String,
    pub mem_size: i32,
}


pub struct Prof {
    counter: u32,
    durations: VecDeque<Duration>,
    args: ProfArgs, 
    start_time: SystemTime,
}


impl Prof {

    pub fn new(args: ProfArgs) -> Prof {
        Prof {
            counter: 1,
            args: args,
            durations: VecDeque::new(),
            start_time: SystemTime::now(),
        }
    }
    

    pub fn just_label(label: &str) -> Prof {
        Prof::new(ProfArgs{
            label: label.to_owned(),
            mem_size: 60*10,
        })
    }
    

    pub fn start(&mut self) {
        self.start_time = SystemTime::now();
    }

 
    // output type is millisecs
    fn calc_variance(&self) -> f64 {
        let mean = self.calc_mean();
        let mut variance = 0.;
        for &duration in &self.durations {
            let diff = duration.as_nanos() as f64 - mean.as_nanos() as f64;
            variance += diff * diff;
        }
        variance /= (self.durations.len() - 1) as f64;
        variance = variance.sqrt();
 
        variance / 1000000.
    }


    fn calc_mean(&self) -> Duration {
        let mut sum = Duration::from_nanos(0);
        for &duration in &self.durations {
            sum += duration;
        }
        let mean = sum / self.durations.len() as u32;
        mean
    }
    

    pub fn pause(&mut self) {
        self.durations.push_back(self.start_time.elapsed().unwrap());
        if self.durations.len() > self.args.mem_size as usize {
            self.durations.pop_front();
        }
        
        if self.counter % super::SKIP_N == 0 {
            println!(
                "{} has duration of {} and variance of {} msecs",
                self.args.label,
                (self.calc_mean().as_nanos() as f64 / 1000000.) as f32,
                self.calc_variance() as f32,
            );
        }
        self.counter += 1;
    }
}
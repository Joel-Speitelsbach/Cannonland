 
use std::collections::VecDeque;
use std::time::{Duration,SystemTime};


pub struct ProfArgs {
    pub label: String,
    pub memSize: i32,
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
            memSize: 60*10,
        })
    }
    

    pub fn start(&mut self) {
        self.start_time = SystemTime::now();
    }

 
    fn calc_variance(&self) -> f64 {
        let mut variance = 0.;
        for &duration in &self.durations {
            let diff = (duration - mean).as_nanos() as f64;
            variance += diff * diff;
        }
        variance /= self.durations.len() as f64;
        variance = variance.sqrt();
 
        variance / 1000000.
    }

    fn calc_mean(&self) -> f64 {
        let mut sum = Duration::from_nanos(0);
        for &duration in &self.durations {
            sum += duration;
        }
        let mean = sum / self.durations.len() as u32;
        mean.as_nanos() 
    }
    
 
    pub fn pause(&mut self) {
        self.durations.push_back(self.start_time.elapsed().unwrap());
        if self.durations.len() > self.args.memSize as usize {
            self.durations.pop_front();
        }
        
        if self.counter % super::SKIP_N == 0 {
            println!(
                "{} has variance of {} msecs",
                self.args.label,
                self.calc_variance(),
            );
        }
        self.counter += 1;
    }
}
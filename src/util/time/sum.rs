use std::time::{Duration,SystemTime};


pub struct ProfArgs {
    pub label: String,
}


pub struct Prof {
    counter: u32,
    duration: Duration,
    args: ProfArgs,
    start_time: SystemTime,
}

impl Prof {
    pub fn new(args: ProfArgs) -> Prof {
        Prof {
            counter: 1,
            args: args,
            duration: Duration::from_millis(0),
            start_time: SystemTime::now(),
        }
    }
    
    pub fn just_label(label: &str) -> Prof {
        Prof::new(ProfArgs{
            label: label.to_owned(),
        })
    }
    
    pub fn start(&mut self) {
        self.start_time = SystemTime::now();
    }
    
    pub fn pause(&mut self) {
        self.duration += self.start_time.elapsed().unwrap();
        
        if self.counter % SKIP_N == 0 {
            println!(
                "{} needed {} msecs",
                self.args.label,
                (self.duration / self.counter).subsec_nanos() as f32
                    / (1000.*1000.),
            );
        }
        self.counter += 1;
    }
}

extern crate sdl2;

use self::sdl2::mixer::{self,Chunk,Channel,Music};
use std::collections::HashMap;


pub struct Sound {
    active: bool,
    sounds: HashMap<String, Chunk>,
}


fn file_list() -> Vec<&'static str> { 
    vec!(
        "crunch.wav",
        "explode.wav",
        "putbomb.wav",
        "schnief.wav",
        "typewriter.wav",
        "whoosh.wav",
    )
}


impl Sound {
    pub fn init() -> Sound {
        mixer::open_audio(
            mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, 
            mixer::DEFAULT_CHANNELS, 256
        ).unwrap();
        Music::set_volume(mixer::MAX_VOLUME / 4);
        Channel::all().set_volume(mixer::MAX_VOLUME / 4);

        let mut sounds = HashMap::new();
        
        for s in file_list() {
            let file_name = format!("sounds/{}", s);
            let chunk = Chunk::from_file(file_name).unwrap();
            sounds.insert(s.to_string(), chunk);
        }

        Sound {
            active: true,
            sounds,
        }
    }

    pub fn stub() -> Sound {
        Sound {
            active: false,
            sounds: HashMap::new(),
        }
    }

    pub fn play(&self, name: &str) {
        if !self.active {return};
        
        let chunk = self.sounds.get(name)
            .expect(&format!("sound file {} is not loaded", name));
        match Channel::all().play(chunk, 0) {
            Err(err) => eprintln!("sdl2_mixer: could not play sound {}: {}", name, err),
            Ok(_) => {},
        }
    }
}


pub fn test() {
    let sound = std::rc::Rc::new(Sound::init());
    let _sound2 = sound.clone();
    for _ in 0..3 {
        for s in file_list() {
            println!("{}", s);
            sound.play(s);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(2000));
}
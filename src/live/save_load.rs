use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

use super::{app::AppData, LiveSettings};

const DEFAULT_SETTINGS_FILE : &str =
"move_force 	   		        100.0
vel_range 					-1.0..1.0
radius_range 				8..20
max_alive 					100.0
start_alive_range 			1.0..100.0
dead_time 					0.0
start_energy 				1.0
division_energy 			10.0
alive_to_energy_coef 		0.1
photosynth_rate 			0.02
carnivore_rate 				10.0
carnivore_damage 			15.0
defence 					15.0
carnivore_cost 				20.0
genome_mut_range 			0.9..1.1
radius_mut_range 			0.9..1.1
flagella_num_range 			6..14
flagella_len_range 			2..8
max_energy_distribution 	10.0
max_repulsive_force 		300.0";

pub fn try_save(app: &AppData) -> std::io::Result<()> {
    if app.frame_num % 1000 == 0 {
        let path = format!("{}/bacteries_data_{}.csv", std::env::current_dir().unwrap().display(), app.frame_num);
        let file = File::create(path).unwrap();
        let mut writer = csv::Writer::from_writer(file);

        let genome = &app.live_data.bacteries.genome;
        let headers = ["live_regen_rate",
                       "division_rate",
                       "photosynth",
                       "carnivore",
                       "movement_force",
                       "movement_rate",
                       "defence",
                       "energy_distribution",
                       "repulsive_force"];
        writer.write_record(&headers)?;
        for i in 0..genome.length {
            let row = [genome.live_regen_rate[i].to_string(),
                       genome.division_rate[i].to_string(),
                       genome.photosynth[i].to_string(),
                       genome.carnivore[i].to_string(),
                       genome.movement_force[i].to_string(),
                       genome.movement_rate[i].to_string(),
                       genome.defence[i].to_string(),
                       genome.energy_distribution[i].to_string(),
                       genome.repulsive_force[i].to_string(),
                       ];
            writer.write_record(&row)?;
        }

        writer.flush()?;
    }
    
    Ok(())
}

pub fn create_default_settings_file(path: &String) {
    match File::create(path.clone()) {
        Ok(mut f) => {
            match f.write_all(DEFAULT_SETTINGS_FILE.as_bytes()) {
                Ok(_) => { println!("Default settings file created ({})", path); },
                Err(e) => log_err(e),
            }
        },
        Err(e) => log_err(e),
    };

    fn log_err(err: std::io::Error) {
        eprintln!("Can`t create default settings file with error: {}", err);
    }
}

pub fn load_settings() -> LiveSettings {
    let path = format!("{}/bac_settings.txt", std::env::current_dir().unwrap().display());
    if !Path::new(path.as_str()).exists(){
        create_default_settings_file(&path);
    }
    let file = File::open(path);
    let mut result = LiveSettings::new();
    let mut floats = [
        ("move_force", &mut result.move_force),

        ("max_alive", &mut result.max_alive),
        ("dead_time", &mut result.dead_time),

        ("start_energy", &mut result.start_energy),
        ("division_energy", &mut result.division_energy),
        ("alive_to_energy_coef", &mut result.alive_to_energy_coef),

        ("photosynth_rate", &mut result.photosynth_rate),
        ("carnivore_rate", &mut result.carnivore_rate),
        ("carnivore_damage", &mut result.carnivore_damage),
        ("defence", &mut result.defence),
        ("carnivore_cost", &mut result.carnivore_cost),

        ("max_energy_distribution", &mut result.max_energy_distribution),

        ("max_repulsive_force", &mut result.max_repulsive_force),
    ];

    let mut ranges_f32 = [
        ("vel_range", &mut result.vel_range),

        ("start_alive_range", &mut result.start_alive_range),

        ("genome_mut_range", &mut result.genome_mut_range),
        ("radius_mut_range", &mut result.radius_mut_range),
    ];

    let mut ranges_i32 = [
        ("radius_range", &mut result.radius_range),

        ("flagella_num_range", &mut result.flagella_num_range),
        ("flagella_len_range", &mut result.flagella_len_range),
    ];

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if line.is_err() { continue; }
                let line = line.unwrap();
                read_floats(&mut floats.iter_mut(), &line);
                read_ranges::<f32>(&mut ranges_f32.iter_mut(), &line);
                read_ranges::<i32>(&mut ranges_i32.iter_mut(), &line);
            }
        },
        Err(err) => eprintln!("Can`t open settings file with error: {}", err),
    }

    result
}

fn read_floats(floats: &mut std::slice::IterMut<(&str, &mut f32)>, line: &String) {
    for (name, field) in floats {
        if line.contains(*name) {
            let value = line
                            .chars()
                            .filter(is_settings_valid_char)
                            .collect::<String>()
                            .parse::<f32>();
            match value {
                Ok(v) => {
                    **field = v;
                    println!("Field {} read, and value set to {}", name, v);
                },
                Err(e) => eprintln!("Can`t parse settings {} with error: {}", name, e),
            }
        }
    }
}

fn read_ranges<T: FromStr + Default + Display + Copy>(ranges: &mut std::slice::IterMut<(&str, &mut std::ops::Range<T>)>, line: &String)
    where <T as FromStr>::Err: std::fmt::Display
{
    for (name, field) in ranges {
        if line.contains(*name) {
            let split = line
                            .chars()
                            .filter(is_settings_valid_char)
                            .collect::<String>();
            let split = split.split("..");

            if split.clone().count() < 2 {
                eprintln!("Can`t parse settings {}, range split is incorrect", name);
                continue;
            }

            let range = read_range_split::<T>(&mut split.clone(), &name);
            (*field).start = range.start;
            (*field).end = range.end;
            println!("Field {} read, and range value set to {}..{}", name, range.start, range.end);
        }
    }
}

fn read_range_split<T: FromStr + Default>(split: &mut std::str::Split<&str>, field_name: &str) -> Range<T>
    where <T as FromStr>::Err: std::fmt::Display
{
    return Range {
        start: read(split, field_name),
        end: read(split, field_name),
    };

    #[inline(always)]
    fn read<T: FromStr + Default>(split: &mut std::str::Split<&str>, field_name: &str) -> T
        where <T as FromStr>::Err: std::fmt::Display
    {
        match split.nth(0) {
            Some(s) => {
                match s.parse::<T>() {
                    Ok(v) => {
                        return v
                    },
                    Err(e) => {
                        eprintln!("Can`t parse settings {} with error: {}", field_name, e);
                    }
                };
            },
            None => {
                eprintln!("Can`t parse settings {}, range split is incorrect", field_name);
            },
        }

        Default::default()
    }
}

fn is_settings_valid_char(c: &char) -> bool {
    c.is_numeric() || *c == '.' || *c == ',' || *c == '-'
}
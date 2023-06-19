
use std::fs::File;

use super::app::AppData;

pub fn try_save(app: &AppData) -> std::io::Result<()> {
    if app.frame_num % 10000 == 0 {
        let path = format!("{}/bacteries_data_{}.csv", std::env::current_dir().unwrap().display(), app.frame_num);
        let file = File::create(path).unwrap();
        let mut writer = csv::Writer::from_writer(file);

        let genome = &app.live_data.bacteries.genome;
        let headers = ["live_regen_rate",
                       "division_rate",
                       "photosynth",
                       "carnivore",
                       "movement_force",
                       "movement_rate"];
        writer.write_record(&headers)?;
        for v in 0..genome.length {
            let row = [genome.live_regen_rate[v].to_string(),
                       genome.division_rate[v].to_string(),
                       genome.photosynth[v].to_string(),
                       genome.carnivore[v].to_string(),
                       genome.movement_force[v].to_string(),
                       genome.movement_rate[v].to_string(),
                       ];
            writer.write_record(&row)?;
        }

        writer.flush()?;
    }
    
    Ok(())
}
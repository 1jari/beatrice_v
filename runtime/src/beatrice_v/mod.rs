#[allow(non_camel_case_types)] 

use colored::Colorize;
use std::fs;
use serde_json::Value;

/* +---------------+ */
/* | Local Modules | */
/* +---------------+ */
pub mod engine;
pub mod project;
pub mod state;

/** @struct Session
 *  @brief  Handles the current project.
 */
pub struct Session {
    directory:  String,         /* Current project directory */
    state:      state::State,   /* Current project State */
}


/** @impl   Session
 *  @func   spawn       - Public Constructor
 *  @func   load_json   - Loads the Robot configuration
 */
impl Session {

    /** @func   load_json
     *  @brief  "Loads the Robot configuration"
     * 
     *  @param  dir     - Json Directory
    */
    fn
    load_json(dir:  &str) -> state::State {

        /* Prepare message */
        println!(   "{} {} {} {}...",
                    "===>".blue(),
                    "Loading".green().underline(),
                    "Robot".purple(),
                    "configuration".yellow());

        /* +-----------------+ */
        /* | Load robot.json | */
        /* +-----------------+ */
        let contents    = fs::read_to_string(dir.to_string()).expect("REASON");
        let json: Value = serde_json::from_str(&contents).expect("REASON");

        /* Metadata */
        let metadata = project::Metadata::new(
            json["metadata"]["name"].as_str().unwrap_or(""),
            json["metadata"]["author"].as_str().unwrap_or(""),
            json["metadata"]["version"].as_str().unwrap_or(""),
            json["metadata"]["license"].as_str().unwrap_or(""),
        );

        /* Rig 
        let rig = json["robot"]["rig"];
        let w   = rig["size"]["x"].as_u64().unwrap_or(0);
        let h   = rig["size"]["y"].as_u64().unwrap_or(0);
        */
        let robot: engine::robot::Robot =
            engine::robot::Robot::new( engine::math::vec::vec2u16_t{ x: 21,  y: 12 } );

        /* Return */
        state::State::new(metadata, robot)
    }

    /** @func   spawn
     *  @brief  "Public Constructor"
     * 
     *  @param  dir     - Project Directory
    */
    pub fn
    spawn(dir:  String) -> Self {

        /* Adjust the directory path */
        let directory   =   dir + "/";

        /* Prepare message */
        println!(   "{} {} @ '{}'",
                    "Spawning".green().underline(),
                    "session".yellow(),
                    directory.blue().underline());

        /* Load robot.json */
        let state       =   Self::load_json(&(directory.to_string() + "robot.json"));

        println!(   "{}", "============================================================".blue());

        println!(   "🤖 {} <{}> - version {} ({})",
                    state.metadata().name.yellow().bold().underline(),
                    state.metadata().author.green(),
                    state.metadata().version.blue(),
                    state.metadata().license.red());

        println!(   ".");


        /* Return */
        Self { directory, state }
    }
}
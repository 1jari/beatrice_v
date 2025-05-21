#[allow(non_camel_case_types)]

/* +---------------+ */
/* | Local Modules | */
/* +---------------+ */
use crate::beatrice_v::project;
use crate::beatrice_v::engine;

/** @struct State 
 *  @brief  "Handles all events"
*/
pub struct State {
    metadata:   project::Metadata,
    robot:      engine::robot::Robot,
}

/** @impl   State 
 *  @func   new         - "Public Constructor"
 *  @func   metadata    - "Metadata Getter"
*/
impl State {

    /** @func   new 
     *  @brief  "Public Constructor"
     * 
     *  @param  metadata -  Project Metadata
     *  @param  robot    -  Main Robot
    */
    pub fn
    new(metadata:   project::Metadata,
        robot:      engine::robot::Robot)   -> Self {
        Self{   metadata:   metadata,
                robot:      robot   }
    }

    /** @func   metadata 
     *  @brief  "Metadata Getter"
    */
    pub fn
    metadata(&self) ->  &project::Metadata {
        &self.metadata
    }
}
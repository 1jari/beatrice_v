/* +----------+ */
/* | Includes | */
/* +----------+ */

/* For fast (and unsafe) Stack allocation */
use std::alloc::{alloc, realloc, dealloc, Layout};
use std::ptr;

/* +---------------+ */
/* | Local modules | */
/* +---------------+ */
use crate::beatrice_v::engine::entity;
use crate::beatrice_v::engine::math;
use crate::beatrice_v::engine::controller as ctrl;

/** @struct Robot
 *  @brief  Main Robot
 */
pub struct Robot {
    m_len:  usize,
    s_len:  usize,
    entity: entity::Entity,
    motor:  Vec<ctrl::motor::Motor>,
    sensor: Vec<ctrl::sensor::Sensor>,
}

/** @impl   Robot
 *  
 *  @func   new       - "Public Constructor"
 *  @func   entity    - "Entity Getter"
 */
impl Robot {
    /** @func   new
     *  @brief  "Public Constructor"
     *  
     *  @param  size    -   Size of the robot
     */
    pub fn new(size: math::vec::vec2u16_t) -> Self {
        let e = entity::Entity::new(size);
        
        Self {
            entity: e,
            m_len:  0,
            s_len:  0,
            motor:  Vec::new(),
            sensor: Vec::new(),
        }
    }
    
    /** @func   entity
     *  @brief  "Entity Getter"
     */
    pub fn entity(&self) -> &entity::Entity {
        &self.entity
    }
}

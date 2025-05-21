/* +---------------+ */
/* | Local Modules | */
/* +---------------+ */
use crate::beatrice_v::engine::math;

/** @struct Entity
 *  @brief  Generic Entity
*/
pub struct Entity {
    axis:   math::axis::Axis,
    size:   math::vec::vec2u16_t,
}

/** @impl   Entity
 * 
 *  @func   new       - "Public Constructor"
 *  @func   position  - "Position Getter"
 *  @func   size      - "Size Getter"
 */
impl Entity {
    
    /** @func   new
     *  @brief  "Public Constructor"
     *  
     *  @param  size    -   Size of the robot
     */
    pub fn
    new(size:   math::vec::vec2u16_t) -> Self {

        /* Create Axis */
        let axis =  math::axis::Axis::new(0, math::vec::vec2i32_t{ x: 0, y: 0 });
        
        Self { size: size, axis: axis }
    }

    /** @func   axis
     *  @brief  "Axis Getter"
     */
    pub fn 
    axis(&self) -> &math::axis::Axis {
        &self.axis
    }

    /** @func   size
     *  @brief  "Size Getter"
     */
    pub fn 
    size(&self)     -> &math::vec::vec2u16_t {
        &self.size
    }
}
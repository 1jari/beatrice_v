use crate::beatrice_v::engine::math::angle;
use crate::beatrice_v::engine::math::vec;

/** @struct Axis
 * 
 *  @brief  "Axis structure"
 */
pub struct  Axis {
    pub ang:    angle::Ang16,
    pub pos:    vec::vec2i32_t,
}

/** @impl   Axis
 * 
 *  @func   new      -  "Public Constructor"
 */
impl    Axis {

    /** @func   new
     *  @brief  "Public Constructor"
     *  
     *  @param  ang   - Angle
     *  @param  pos   - Position
     */
    pub fn
    new(ang:    angle::Ang16,
        pos:    vec::vec2i32_t)   ->  Self {
        Self { ang: ang, pos: pos }
    }
}
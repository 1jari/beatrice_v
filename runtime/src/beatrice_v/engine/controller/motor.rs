use crate::beatrice_v::engine::controller::driver;

pub enum    MotorKind {
    DC_MOTOR,
    SERVO_MOTOR,
    TACHO_MOTOR,
}

/** @struct Motor
 *  @brief  "Generic Motor handler"
 */
pub struct  Motor {
    kind:   MotorKind,          /* Type of motor */
    driver: driver::Driver,     /* Driver */
}

/** @impl   Motor
 * 
 *  @func   new     -   "Public Constructor"
 *  @func   kind    -   "Kind Getter"
 */
impl Motor {

    /** @func   new
     *  @brief  "Public Constructor"
     * 
     *  @param  kind    -   Type of motor
     *  @param  driver  -   Driver
     */
    pub fn
    new(kind:   MotorKind,
        driver: driver::Driver) ->  Self {
        Self { kind: kind, driver: driver }
    }

    /** @func   kind
     *  @brief  "Kind Getter"
     */
    pub fn
    kind(&self) ->  &MotorKind {
        &self.kind
    }
}
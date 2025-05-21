use crate::beatrice_v::engine::controller::driver;

/** @struct Sensor
 *  @brief  "Generic Motor handler"
 */
pub struct  Sensor {
    alias:  String,             /* Type of motor */
    driver: driver::Driver,     /* Driver */
}

/** @impl   Sensor
 * 
 *  @func   new     -   "Public Constructor"
 *  @func   alias   -   "Alias Getter"
 */
impl Sensor {

    /** @func   new
     *  @brief  "Public Constructor"
     * 
     *  @param  alias   -   Alias
     *  @param  driver  -   Driver
     */
    pub fn
    new(alias:  String,
        driver: driver::Driver) ->  Self {
        Self { alias: alias, driver: driver }
    }

    /** @func   alias
     *  @brief  "Alias Getter"
     */
    pub fn
    alias(&self) ->  &String {
        &self.alias
    }
}
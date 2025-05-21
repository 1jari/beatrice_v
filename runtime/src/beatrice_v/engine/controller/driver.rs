/** @trait  Driver
 *  @brief  "Handles all device communications"
 */
pub struct Driver {
    pub signal:     [u32; 2],   /* I/O Signal */
        addr:       u32,        /* Address */
}

/** @impl   Driver
 *  
 *  @func   new     - "Public Constructor"
 *  @func   addr    - "Adress Getter"
 *  @func   signal  - "Signal Getter"
*/
impl Driver {

    /** @func   new
     *  @brief  "Public Constructor"
     * 
     *  @param  addr    - Adress
     *  @param  signal  - I/O Signal
     */
    pub fn
    new(addr:   u32,
        signal: [u32; 2])   -> Self {
        Self { addr: addr, signal: signal }
    }
}

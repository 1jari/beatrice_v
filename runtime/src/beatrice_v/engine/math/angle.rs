/* +------------------------------------------+ */
/* | Binary Ang16 Measurement constants (BAM) | */
/* +------------------------------------------+ */

/* Ang16s are represented as 32-bit fixed point numbers where:
 * - 0x00000000 = 0 degrees
 * - 0x20000000 = 45 degrees
 * - 0x40000000 = 90 degrees
 * - 0x80000000 = 180 degrees
 * - 0xC0000000 = 270 degrees
 * - 0xFFFFFFFF ≈ 359.999999 degrees
 */
pub const ANG45:    Ang16   = 0x2000;
pub const ANG90:    Ang16   = 0x4000;
pub const ANG180:   Ang16   = 0x8000;
pub const ANG270:   Ang16   = 0xC000;
pub const ANG359:   Ang16   = 0xEFFF;

/* @type    Ang16
 * 
 * @brief   "Binary Angle Measurement type (32-bit fixed point)"
 */
pub type Ang16 = i32;

/* @func    ang_from_deg
 * @brief   "Convert degrees to Binary Angle Measurement (BAM)"
 * 
 * @param   deg -   Angle in degrees
 */
pub fn
ang_from_deg(deg: f32) -> Ang16 {
    /* Normalize the degree value first */
    let normalized_deg = deg % 360.0;

    /* Convert to BAM (ANG359 = 360 degrees) */
    ((normalized_deg) * ((ANG359 as f32)/ 180.0)) as Ang16
}

/* @func    ang_to_deg
 * @brief   "Convert Binary Ang16 Measurement (BAM) to degrees"
 * 
 * @param   ang -   Ang16 in BAM
 */
pub fn
ang_to_deg(ang: Ang16) -> f32 {
    /* Convert to degrees (180 degrees = 0x8000) */
    ((ang as f32) * (180.0 / (ANG180 as f32))) as f32
}
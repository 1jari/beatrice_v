#[allow(non_camel_case_types)]

/** @struct Metadata 
 *  @brief  "Stores metadata"
*/
pub struct Metadata {
    pub name:       String,
    pub author:     String,
    pub version:    String,
    pub license:    String,
}
/** @impl   Metadata
 *  @func   new         - "Public Constructor"
 */
impl Metadata {

    /** @func   new
     *  @brief  "Public Constructor"
     * 
     *  @param  name
     *  @param  author
     *  @param  version
     *  @param  license
     */
    pub fn
    new(name:       &str,
        author:     &str,
        version:    &str,
        license:    &str) -> Self {
        Self{   name:       name.to_string(), 
                author:     author.to_string(), 
                version:    version.to_string(), 
                license:    license.to_string() }
    }
}

use csv_to_usv::csv_to_usv as to_usv;
use extism_pdk::*;

#[plugin_fn]
pub fn csv_to_usv(csv: String) -> FnResult<String> {
    Ok(to_usv(&csv))
}

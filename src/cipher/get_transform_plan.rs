use super::get_initial_function_name;
use crate::{helpers::regex_search, AppResult};
use std::vec::Vec;

pub fn get_transform_plan(js: &str) -> AppResult<Vec<String>> {
    let fn_name = get_initial_function_name(js)?;
    let name = fancy_regex::escape(&fn_name);
    let pattern = format!(r#"{}=function\(\w\)[a-z=\.\("\)]*;(.*);(?:.+)"#, name);
    log::debug!("getting transform plan");
    let transform_plan_str = regex_search(&pattern, js, 1)?;
    Ok(transform_plan_str
        .split(';')
        .map(|s| s.trim().to_string())
        .collect())
}

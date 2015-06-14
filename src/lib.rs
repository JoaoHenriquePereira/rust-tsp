extern crate libc;
extern crate rand;
extern crate rustc_serialize;

pub mod rusttsp;

use libc::c_char;
use rustc_serialize::json::{self, Json, ToJson};
use rusttsp::graph::Graph;
use std::collections::BTreeMap;
use std::ffi::{CString, CStr};
use std::str;

/// External adapter using ffi
#[no_mangle]
pub extern "C" fn compute_adapter(input: *const c_char) -> *const c_char {
    let c_input = unsafe { CStr::from_ptr(input).to_bytes() };
    match str::from_utf8(c_input) {
        Ok(input) => compute(input),
        Err(e) => CString::new(e.to_string()).unwrap().as_ptr(),
    }
}

// Actual handling
fn compute(input: &str) -> *const c_char {
    
    let mut result: &str = "etc";

    CString::new(result).unwrap().as_ptr()
}

/// Aux input JSON structs
struct Input {
	graph_type: &'static str,
	graph: Graph,
	options: InputOptions,
}

struct InputOptions {
	mutation_rate: f64,
	elitism: bool,
	population_size: usize,
	tournament_size: usize,
}

impl ToJson for Input {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("graph_type".to_string(), self.graph_type.to_json());
        d.insert("graph".to_string(), self.graph.to_json());
        d.insert("options".to_string(), self.options.to_json());
        Json::Object(d)
    }
}

impl ToJson for InputOptions {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("mutation_rate".to_string(), self.mutation_rate.to_json());
        d.insert("elitism".to_string(), self.elitism.to_json());
        d.insert("population_size".to_string(), self.population_size.to_json());
        d.insert("tournament_size".to_string(), self.tournament_size.to_json());
        Json::Object(d)
    }
}
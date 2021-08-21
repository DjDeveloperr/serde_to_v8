use serde_json::{Map, Number, Value, from_str};
use wasm_bindgen::prelude::*;
use integer_encoding::VarInt;
use convert_case::{Case, Casing};

#[inline(always)]
pub fn push_null(buf: &mut Vec<u8>) {
    buf.push(48);
}

#[inline(always)]
pub fn push_bool(buf: &mut Vec<u8>, val: bool) {
    buf.push(if val == true { 84 } else { 70 });
}

#[inline(always)]
pub fn push_i32(buf: &mut Vec<u8>, val: i32) {
    buf.push(73);
    let mut enc = val.encode_var_vec();
    buf.append(&mut enc);
}

#[inline(always)]
pub fn push_u32(buf: &mut Vec<u8>, val: u32) {
    buf.push(85);
    let mut enc = val.encode_var_vec();
    buf.append(&mut enc);
}

#[inline(always)]
pub fn push_f64(buf: &mut Vec<u8>, val: f64) {
    buf.push(78);
    buf.append(&mut val.to_ne_bytes().to_vec());
}

#[inline(always)]
pub fn push_i64(_: &mut Vec<u8>, _: i64) {
    // buf.push(90);
    // I don't think JSON needs this
    unimplemented!();
}

#[inline(always)]
pub fn push_number(buf: &mut Vec<u8>, num: Number) {
    if num.is_f64() {
        // Double
        push_f64(buf, num.as_f64().unwrap());
    } else if num.is_i64() && num.as_i64().unwrap() <= i32::MAX as i64 {
        // Int
        push_i32(buf, num.as_i64().unwrap() as i32);
    } else if num.is_u64() && num.as_u64().unwrap() <= u32::MAX as u64 {
        // Unsigned Int
        push_u32(buf, num.as_u64().unwrap() as u32);
    } else {
        // BigInt
        push_i64(buf, num.as_i64().unwrap());
    }
}

#[inline(always)]
pub fn push_string(buf: &mut Vec<u8>, mut val: String) {
    buf.push(34);
    let mut enc = val.len().encode_var_vec();
    buf.append(&mut enc);
    buf.append(unsafe { val.as_mut_vec() });
}

#[inline(always)]
pub fn push_array(buf: &mut Vec<u8>, arr: Vec<Value>) {
    buf.push(65);
    let len = arr.len();
    let enc = len.encode_var_vec();
    for byte in &enc {
        buf.push(*byte);
    }
    for val in arr {
        push_value(buf, val);
    }
    buf.push(36);
    buf.push(0);
    for byte in enc {
        buf.push(byte);
    }
}

pub fn push_object(buf: &mut Vec<u8>, obj: Map<String, Value>) {
    buf.push(111);
    let mut written: u32 = 0;
    for (k, v) in obj {
        push_string(buf, k.to_case(Case::Camel));
        push_value(buf, v);
        written += 1;
    }
    buf.push(123);
    let mut enc = written.encode_var_vec();
    buf.append(&mut enc);
}

pub fn push_value(buf: &mut Vec<u8>, v: Value) {
    match v {
        Value::Null => push_null(buf),
        Value::Bool(val) => push_bool(buf, val),
        Value::Number(num) => push_number(buf, num),
        Value::String(val) => push_string(buf, val),
        Value::Array(arr) => push_array(buf, arr),
        Value::Object(obj) => push_object(buf, obj)
    }
}

#[wasm_bindgen]
pub fn json_to_v8(json: &str) -> Vec<u8> {
    let value: Value = from_str(json).unwrap();
    let mut result = vec![0xFF, 0xD];
    push_value(&mut result, value);
    result
}

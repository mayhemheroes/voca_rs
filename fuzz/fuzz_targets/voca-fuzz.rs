#![no_main]
use libfuzzer_sys::fuzz_target;
use std::str;
use voca_rs::*;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0 {
        let opt = data[0];
        match str::from_utf8(&data[1..]) {
            Ok(in_string)=>{
                match opt {
                    0=>{
                        in_string._foreign_key();
                    },
                    1=>{
                        in_string._is_alpha();
                    },
                    2=>{
                        in_string._is_capitalize();
                    },
                    3=>{
                        in_string._count_graphemes();
                    },
                    4=>{
                        in_string._escape_html();
                    },
                    5=>{
                        in_string._escape_regexp();
                    },
                    6=>{
                        in_string._unescape_html();
                    },
                    7=>{
                        in_string._latinise();
                    },
                    8=>{
                        in_string._reverse();
                    },
                    9=>{
                        in_string._slugify();
                    },
                    10=>{
                        in_string._is_digit();
                    },
                    11=>{
                        in_string._is_shouty_kebab_case();
                    },
                    12=>{
                        in_string._is_title();
                    },
                    13=>{
                        in_string._max_code_point();
                    },
                    _=>()
                }
            },
            Err(..)=>()
        }
    }
});

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
                        in_string._max_code_point();
                    },
                    2=>{
                        in_string._min_code_point();
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
                    _=>()
                }
            },
            Err(..)=>()
        }
    }
});

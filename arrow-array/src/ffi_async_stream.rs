// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::ffi::{c_char, c_int, c_longlong, c_void};

use arrow_data::ffi::FFI_ArrowArray;
use arrow_schema::ffi::FFI_ArrowSchema;

#[allow(non_camel_case_types)]
pub type FFI_ArrowDeviceType = c_int;

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct FFI_ArrowDeviceArray {
    pub array: FFI_ArrowArray,
    pub device_id: c_longlong,
    pub device_type: FFI_ArrowDeviceType,
    pub sync_event: *mut c_void,
    pub reserved: [c_longlong; 3],
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct FFI_ArrowAsyncTask {
    pub extract_data:
        Option<unsafe extern "C" fn(arg1: *mut Self, out: *mut FFI_ArrowDeviceArray) -> c_int>,
    pub private_data: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct FFI_ArrowAsyncProducer {
    pub device_type: FFI_ArrowDeviceType,
    pub request: Option<unsafe extern "C" fn(arg1: *mut Self, n: c_longlong) -> c_void>,
    pub cancel: Option<unsafe extern "C" fn(arg1: *mut Self) -> c_void>,
    pub additional_metadata: *const c_char,
    pub private_data: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct FFI_ArrowAsyncDeviceStreamHandler {
    pub on_schema:
        Option<unsafe extern "C" fn(arg1: *mut Self, stream_schema: *mut FFI_ArrowSchema) -> c_int>,
    pub on_next_task: Option<
        unsafe extern "C" fn(
            arg1: *mut Self,
            task: *mut FFI_ArrowAsyncTask,
            metadata: *const c_char,
        ) -> c_int,
    >,
    pub on_error: Option<
        unsafe extern "C" fn(
            arg1: *mut Self,
            code: c_int,
            message: *const c_char,
            metadata: *const c_char,
        ) -> c_void,
    >,
    pub release: Option<unsafe extern "C" fn(arg1: *mut Self) -> c_void>,
    pub producer: *mut FFI_ArrowAsyncProducer,
    pub private_data: *mut c_void,
}

// Copyright 2018 The Rusix Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate capnpc;
extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src",
        input: &["protos/err.proto", "protos/service.proto"],
        includes: &["protos"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    }).expect("protoc");

    ::capnpc::CompilerCommand::new()
        .file("protos/service.capnp")
        .run()
        .expect("compiling schema");
}

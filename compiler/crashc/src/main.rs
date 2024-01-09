/*
 * Copyright 2024 Julian Siebert
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::Parser;

fn main() {
    let args = Args::parse();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The target Crash file you want to compile
    #[arg(short)]
    target_file_name: String,
    /// If only the llvm ir should be generated
    #[arg(short)]
    llvm_ir_only: bool,
    /// If only the assembly should be generated
    #[arg(short)]
    asm_only: bool
}
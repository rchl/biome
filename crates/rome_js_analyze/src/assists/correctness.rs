//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod flip_bin_exp;
mod inline_variable;
mod organize_imports;
declare_group! { pub (crate) Correctness { name : "correctness" , rules : [self :: flip_bin_exp :: FlipBinExp , self :: inline_variable :: InlineVariable , self :: organize_imports :: OrganizeImports ,] } }

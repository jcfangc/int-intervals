mod signed;
mod unsigned;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

const UNSIGNED_TYPES: &[(&str, &str)] = &[
    ("u16", "U16CO"),
    ("u32", "U32CO"),
    ("u64", "U64CO"),
    ("u128", "U128CO"),
    ("usize", "UsizeCO"),
];

const SIGNED_TYPES: &[(&str, &str)] = &[
    ("i16", "I16CO"),
    ("i32", "I32CO"),
    ("i64", "I64CO"),
    ("i128", "I128CO"),
    ("isize", "IsizeCO"),
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum EmitMode {
    Write,
    Check,
}

impl EmitMode {
    #[inline]
    fn is_check(self) -> bool {
        matches!(self, Self::Check)
    }
}

fn main() {
    let mut args = env::args().skip(1);

    let mode = args.next().unwrap_or_else(|| {
        panic!("missing mode, expected one of: `signed`, `unsigned`, `all`, `check`")
    });

    let mut emit = EmitMode::Write;

    let target = match mode.as_str() {
        "signed" | "unsigned" | "all" => mode.as_str(),
        "check" => {
            emit = EmitMode::Check;
            "all"
        }
        other => panic!(
            "unsupported mode: {other}, expected one of: `signed`, `unsigned`, `all`, `check`"
        ),
    };

    for arg in args {
        match arg.as_str() {
            "--check" | "check" => emit = EmitMode::Check,
            other => panic!("unsupported arg: {other}, expected `--check`"),
        }
    }

    let root = workspace_root();
    let src = root.join("src").join("interval");

    match target {
        "signed" => signed::generate(&src, emit),
        "unsigned" => unsigned::generate(&src, emit),
        "all" => {
            signed::generate(&src, emit);
            unsigned::generate(&src, emit);
        }
        _ => unreachable!(),
    }

    write_interval_mod(&src, emit);

    if emit.is_check() {
        eprintln!("codegen check passed");
    }
}

/// workspace root
fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // xtask
        .unwrap()
        .parent() // workspace root
        .unwrap()
        .to_path_buf()
}

pub(crate) fn emit_file(path: impl AsRef<Path>, expected: String, mode: EmitMode) {
    let path = path.as_ref();

    match mode {
        EmitMode::Write => {
            fs::write(path, expected).unwrap();
        }
        EmitMode::Check => {
            let actual = fs::read_to_string(path).unwrap_or_else(|err| {
                panic!(
                    "generated file is missing or unreadable: {}\nerror: {err}",
                    path.display()
                )
            });

            if actual != expected {
                panic!(
                    "generated file is stale: {}\nrun codegen and commit the updated output",
                    path.display()
                );
            }
        }
    }
}

fn write_interval_mod(src: &Path, mode: EmitMode) {
    // Build sorted lists — cargo fmt sorts mod declarations alphabetically.
    let mut mods: Vec<String> = Vec::new();
    mods.push("mod i128;".into());
    mods.push("mod i16;".into());
    mods.push("mod i32;".into());
    mods.push("mod i64;".into());
    mods.push("mod i8;".into());
    mods.push("mod isize;".into());
    mods.push("mod res;".into());
    mods.push("pub mod traits;".into());
    mods.push("mod u128;".into());
    mods.push("mod u16;".into());
    mods.push("mod u32;".into());
    mods.push("mod u64;".into());
    mods.push("mod u8;".into());
    mods.push("mod usize;".into());

    let mut s = String::new();
    for m in &mods {
        s.push_str(m);
        s.push('\n');
    }
    s.push('\n');

    // Re-export order must match cargo fmt numeric sort:
    // i8, i16, i32, i64, i128, isize, res, u8, u16, u32, u64, u128, usize
    s.push_str("pub use i8::I8CO;\n");
    s.push_str("pub use i16::I16CO;\n");
    s.push_str("pub use i32::I32CO;\n");
    s.push_str("pub use i64::I64CO;\n");
    s.push_str("pub use i128::I128CO;\n");
    s.push_str("pub use isize::IsizeCO;\n");
    s.push_str("pub use res::{EmptyRangeError, OneTwo, ZeroOneTwo};\n");
    s.push_str("pub use u8::U8CO;\n");
    s.push_str("pub use u16::U16CO;\n");
    s.push_str("pub use u32::U32CO;\n");
    s.push_str("pub use u64::U64CO;\n");
    s.push_str("pub use u128::U128CO;\n");
    s.push_str("pub use usize::UsizeCO;\n");

    emit_file(src.join("..").join("interval.rs"), s, mode);
}

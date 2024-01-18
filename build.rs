use std::io::{BufWriter, Result, Write};

#[derive(Clone, Copy)]
enum BitInfo {
    Unsigned {
        prim_max: u64,
    },
    Signed {
        signed_max: i64,
        signed_min: i64,
        prim_max: i64,
    },
}

struct EnumInfo {
    ty_name: &'static str,
    name: &'static str,
    bit_info: BitInfo,
}

const FILES: &[EnumInfo] = &[
    EnumInfo {
        ty_name: "u8",
        name: "u8_repr.rs",
        bit_info: BitInfo::Unsigned {
            prim_max: u8::MAX as _,
        },
    },
    EnumInfo {
        ty_name: "i8",
        name: "i8_repr.rs",
        bit_info: BitInfo::Signed {
            signed_max: i8::MAX as _,
            signed_min: i8::MIN as _,
            prim_max: u8::MAX as _,
        },
    },
    #[cfg(feature = "enum_repr_16")]
    EnumInfo {
        ty_name: "u16",
        name: "u16_repr.rs",
        bit_info: BitInfo::Unsigned {
            prim_max: u16::MAX as _,
        },
    },
    #[cfg(feature = "enum_repr_16")]
    EnumInfo {
        ty_name: "i16",
        name: "i16_repr.rs",
        bit_info: BitInfo::Signed {
            signed_max: i16::MAX as _,
            signed_min: i16::MIN as _,
            prim_max: u16::MAX as _,
        },
    },
];

fn generate_variants(
    mut generated_file: impl Write,
    repr_name: &str,
    bit_info: BitInfo,
) -> Result<()> {
    write!(
        generated_file,
        "#[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[allow(dead_code)]
        pub(crate) enum {} {{",
        repr_name
    )?;

    match bit_info {
        BitInfo::Unsigned { prim_max } => {
            for i in 0..prim_max {
                write!(generated_file, "V{},", i)?
            }
        }
        BitInfo::Signed {
            signed_max,
            signed_min,
            prim_max,
        } => {
            for i in 0..signed_max {
                write!(generated_file, "V{},", i)?;
            }

            for (i, v) in (signed_max..prim_max).zip(signed_min..0) {
                write!(generated_file, "MV{}={},", i, v)?;
            }
        }
    }

    write!(generated_file, "}}")?;
    Ok(())
}

fn generate_impl(mut generated_file: impl Write, repr_name: &str, ty_name: &str) -> Result<()> {
    write!(
        generated_file,
        "impl {} {{
            pub(crate) const fn new(value: {}) -> Option<Self> {{
                unsafe {{ std::mem::transmute(value) }}
            }}
        }}",
        repr_name, ty_name
    )
}

fn main() -> Result<()> {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut open_options = std::fs::OpenOptions::new();
    open_options.create(true).write(true).truncate(true);

    for file in FILES {
        let file_path = format!("{}/{}", out_dir, file.name);
        let mut generated_file = BufWriter::new(open_options.open(file_path)?);

        let repr_name = format!("{}Repr", file.ty_name.to_uppercase());

        generate_variants(&mut generated_file, &repr_name, file.bit_info)?;
        generate_impl(&mut generated_file, &repr_name, file.ty_name)?;

        generated_file.flush()?;
    }

    Ok(())
}

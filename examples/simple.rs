#![allow(unused)]

use derive_enum_accessors::EnumFieldAccessors;

#[derive(EnumFieldAccessors)]
enum IdenticalFields {
    V1 { ivalue: i32 },
    V2 { ivalue: i32 },
    V3 { ivalue: i32 },
}

#[derive(EnumFieldAccessors)]
enum MixedFields {
    V1 { ivalue: i32 },
    V2 { fvalue: f32 },
}

fn main() {
    let identical = IdenticalFields::V1 { ivalue: 1 };
    assert_eq!(*identical.ivalue(), 1);

    let mixed = MixedFields::V1 { ivalue: 1 };
    assert_eq!(mixed.ivalue().cloned(), Some(1));

    let mut identical = IdenticalFields::V2 { ivalue: 2 };

    *identical.ivalue_mut() = 3;
    assert_eq!(*identical.ivalue(), 3);
}

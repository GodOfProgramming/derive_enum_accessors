#![allow(unused)]

use derive_enum_accessors::EnumFieldAccessors;
use std::fmt::Debug;

#[derive(EnumFieldAccessors, Debug)]
enum EqualFields {
    V1 { ivalue: i32 },
    V2 { ivalue: i32 },
    V3 { ivalue: i32 },
}

#[derive(EnumFieldAccessors, Debug)]
enum DifferentFields {
    V1 { ivalue: i32 },
    V2 { fvalue: f32 },
}

#[derive(EnumFieldAccessors, Debug)]
enum GenericFields<T, U>
where
    T: Debug,
    U: Debug,
{
    V1 { value_t: T },
    V2 { value_t: T, value_u: U },
}

#[derive(EnumFieldAccessors, Debug)]
enum LifetimeFields<'a> {
    V1 { value_ref: &'a i32 },
}

type AliasedI32 = i32;

#[derive(EnumFieldAccessors, Debug)]
enum TypeAliasesAreUnsupported {
    V1 { value: i32 },
    V2 { value: AliasedI32 },
}

fn main() {
    let equal = EqualFields::V1 { ivalue: 1 };

    let different = DifferentFields::V1 { ivalue: 1 };

    let mut mut_equal = EqualFields::V2 { ivalue: 0 };
    *mut_equal.ivalue_mut() = 1;

    let lifetime = LifetimeFields::V1 { value_ref: &1 };

    let generic = GenericFields::V2 {
        value_t: 123,
        value_u: "U Value",
    };

    let aliased = TypeAliasesAreUnsupported::V1 { value: 1 };

    println!("{:?}", equal.ivalue());
    println!("{:?}", different.ivalue());
    println!("{:?}", mut_equal.ivalue());
    println!("{:?}", lifetime.value_ref());
    println!("{:?}, {:?}", generic.value_t(), generic.value_u());
    println!("{:?}", aliased /* Can't access a .value() at all */);
}

use derive_enum_accessors::EnumFieldAccessors;

#[test]
fn equal_fields_can_directly_access() {
    #[derive(EnumFieldAccessors)]
    enum Enum {
        V1 { ivalue: i32 },
        V2 { ivalue: i32 },
    }

    let v1 = Enum::V1 { ivalue: 1 };
    let v2 = Enum::V2 { ivalue: 2 };

    assert_eq!(v1.ivalue(), &1);
    assert_eq!(v2.ivalue(), &2);
}

#[test]
fn different_fields_return_options() {
    #[derive(EnumFieldAccessors)]
    enum Enum {
        V1 { ivalue: i32 },
        V2 { fvalue: f32 },
    }

    let v1 = Enum::V1 { ivalue: 1 };
    let v2 = Enum::V2 { fvalue: 2.0 };

    assert_eq!(v1.ivalue(), Some(&1));
    assert_eq!(v2.fvalue(), Some(&2.0));
}

#[test]
fn generics_are_supported() {
    #[derive(EnumFieldAccessors)]
    enum Enum<T, U> {
        V1 { t_value: T },
        V2 { t_value: T, u_value: U },
    }

    let v1 = Enum::<i32, ()>::V1 { t_value: 1 };
    let v2 = Enum::V2 {
        t_value: 2,
        u_value: 3.0,
    };

    assert_eq!(v1.t_value(), &1);
    assert_eq!(v2.t_value(), &2);
    assert_eq!(v2.u_value(), Some(&3.0));
}

#[test]
fn all_together() {
    #[derive(EnumFieldAccessors)]
    enum Enum<'a, T> {
        V1 {
            value: &'a T,
            optional_v1: Option<i32>,
        },
        V2 {
            value: &'a T,
        },
    }

    let v1 = Enum::V1 {
        value: &1,
        optional_v1: Some(0),
    };
    let v2 = Enum::V2 { value: &1.0 };

    assert_eq!(v1.value(), &&1);
    assert_eq!(v1.optional_v1(), Some(&Some(0)));
    assert_eq!(v2.value(), &&1.0);
}

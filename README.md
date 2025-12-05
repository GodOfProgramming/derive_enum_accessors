# Derive Enum Accessors

This crate provides the ability to safely pull out values from enum struct variants so long as they share the same name and type.

This turns into returning a reference to a field guaranteed to exist, and an option for fields that may exist.

If two or more variants share the same field name, but at least one type is different, then it will be excluded and a manual approach will instead be required.

## Why Not Tuples?

I have the belief that if anyone finds `some_enum._0()` or worse `some_enum._0_mut()` to be readable, six months later they'll find out otherwise and this decision serves to help people from frustrating their future selves.

## Limitations

If a type does not exactly match, it will not have a method produced, this means you cannot hypothetically mix `std::vec::Vec` and `Vec` and have this work. Derive macros don't have access to type resolution, so one of the few simple ways to compare types is to stringify it and use that.

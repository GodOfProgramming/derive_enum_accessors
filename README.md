# Derive Enum Accessors

This crate provides functionality similar to TS where if an object implements an interface, you can safely use that field, otherwise you should nullcheck.

In rust terms, this turns into returning a reference to a field guaranteed to exist, and an option for fields that may exist.

If two or more variants share the same field name, but at least one type is different, then it will be excluded and classic means of accessing will be needed.

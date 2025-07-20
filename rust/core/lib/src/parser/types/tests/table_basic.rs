#![cfg(test)]

use crate::parser::types::tests::macros::parser_test_case;

parser_test_case!(
    test_parse_invalid_syntax,
    r#"
        &
    "#,
    false
);

parser_test_case!(
    test_parse_expression_with_single_key,
    r#"
        key = "value"
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_multiple_keys,
    r#"
        key1 = "value1"
        key2 = "value2"
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_variant,
    r#"
        key&v1:1 = "value"
    "#,
    true
);

parser_test_case!(
    test_parser_expression_with_variants,
    r#"
        key = "test"
        key&v1:1 = "test1"
        key&v1:1&v2:2 = "test2"
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_single_dot_key,
    r#"
        parent0.a1.a2.a3 = "test"
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_multiple_dot_keys,
    r#"
        parent0.a1.a2.a3 = "test"
        parent0.a1.a5 = "test"
    "#,
    true
);

// todo: need to verify the data inside but not just Reuslt
parser_test_case!(
    test_parse_expression_with_dot_keys,
    r#"
        parent0.a1.a2.a3 = "test"
        parent0.a1.a5 = "test"
        parent0.a1.a6 = "test"
        parent0.a1.a7 = "test"
        parent1 = "test1"
        parent2 = "test2"
        parent3 = "test3"
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_sub_table,
    r#"
        [sub_table.keys.key2&v1:1]
        key1 = "value1"
        key2 = "value2"
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_multiple_sub_tables,
    r#"
        [sub_table.keys.key2&v1:1]
        key1 = "value1"
        key2 = "value2"
        [sub_table.keys.key2&v1:2]
        key1 = "value1"
        key2 = "value2"
    "#,
    true
);

/*parser_test_case!(
    test_parse_full,
    r#"
        #note:
        #key: must be not quotated-string.
        #simple value: must quotated-string, validated by deserializer.
        #array: must be value
        global_string_key&variant1:v1 = "test"
        global_string_key = "test"
        global_integer_key&variant2:v2 = "5"
        global_integer_key = "5"
        global_float_key = "3.14"
        global_boolean_key = "true"
        global_datetime_key = "2023-10-01T12:00:00Z"
        inlined_simple_array = < "test1", "test2" >
        inlined_simple_integer_array = < "1", "2", "3" >
        inlined_simple_float_array = < "1.1", "2.2", "3.3" >
        inlined_simple_boolean_array = < "true", "false", "true" >
        inlined_simple_datetime_array = < "2023-10-01T12:00:00Z", "2023-10-02T12:00:00Z" >
        inlined_complex_array = < { key1 = "value1", key2 = "10" }, { key1 = "value2", key2 = "20" } >
        inlined_array_of_array = <<"1", "2">,<"3", "4">>
        inlined_entry = { inlined_string_key = "test", inlined_integer_key = "10" }

        # table: [key like string, could have subkeys but last one to have variant]
        [sub_config.b&v1:1]
        sub_string_key = "test_sub"
        sub_integer_key = "80"

        # array <array-key-like_string>
        [test.a&v1:1]
        <<"1", "2">,<"3", "4">>
    "#,
    true
);*/

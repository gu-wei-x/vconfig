#![cfg(test)]

use crate::parser::types::tests::macros::parser_test_case;

parser_test_case!(
    test_parse_expression_with_array,
    r#"
        array1 = <"a1","a2"> #simple array1
        array2 = <"a3","a4"> #simple array2
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_array_of_arrays,
    r#"
        array1 = <<"a1", "a2">>
        array2 = <<"a1", "a2">, <"a3", "a4">>
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_array_of_table,
    r#"
        array1 = <{a1="test", a2="test"}>
        array1&v1:1 = <{a1="test", a2="test"}>
        array2 = <{a1="test", a2="test"}, {a1="test", a2="test"}>
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_sub_array,
    r#"
        [sub_array.keys.key2&v1:1]
        <"value1", "value2">
    "#,
    true
);

parser_test_case!(
    test_parse_expression_with_multiple_sub_arrays,
    r#"
        [sub_array.keys.key2&v1:1]
        <"value1", "value2">
        [sub_array.keys.key2&v1:2]
        <"value3", "value4">
    "#,
    true
);

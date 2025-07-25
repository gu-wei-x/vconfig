# Types and Configuration format.

## what's a varaint.
A variant is a key-value pair means sth is present or true when evaluating an expresstionm.
eg: ```msg&v:1 = "hello world"```, here varaint is ```(name:v, value:1)``` means it's present/true gives ```"hello world"``` to ```msg```. 
Variants means a collection of variants used to evaluate an expession like: ```msg&v1:1&v2:1&... = "hello world"```. variants crate would parse
variant expression to intenal data format from configuration and leverage them to evaluate the value when deserialized configuration content to 
Rust data with a exising variant collection provided by context. An expression with vraints in BNF:
```
<key><varaint exp>* ::= <exp value>
<key> ::= <no-quoted string><.no-quoted string>*
<varaint exp> ::= &<key>:<variant value>
```

## ```<key>``` supported in variants:
### No dot in key
  * Configuration: key is interpreted as the nearest field of a struct.
  * Example:
```
#[derive(Deserialize)]
#[serde(crate = "variants::serde")]
struct Config {
    key: String,
}

let cofig_str = r#"
    key&v1:1 = "v1" #with variants
    key = "v2"
"#;
``` 

### dots in key
  * Configuration: key is interpreted as nested field of a struct.
  * Example:
```
#[derive(Deserialize)]
#[serde(crate = "variants::serde")]
struct Config {
    pkey: SubConfig,
}

#[derive(Deserialize)]
#[serde(crate = "variants::serde")]
struct SubConfig {
    skey: String,
}


let cofig_str = r#"
    pkey.skey&v1:1 = "v1" #with variants
    pkey.skey = "v2"
"#;
``` 

## ```<exp value>``` supported in variants:
  * **primitive types**
  * **option**
  * **array**
  * **struct**

### primitive types
  * Rust types: ```bool f32 f64 i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 String```
  * Configuration: value must be quoted string which could be parsed to the corresponding type.
  * Example:
```
#[derive(Deserialize)]
#[serde(crate = "variants::serde")]
struct Config {
    key1: String,
    key2: u64,
}

let cofig_str = r#"
    key1&v1:1 = "v1" #with variants
    key1 = "v2"
    key2&v1:1 = "5" #with variants
    key2 = "3"
"#;
``` 

### options
  * Rust types: ```Option<T>```, ```T``` could be premitive types, array or struct.
  * Configuration: a missing value will be evaluated to ```None```, valid value will be evaluated to ```Some(T)```.
  * Example:
```
#[derive(Deserialize)]
#[serde(crate = "variants::serde")]
struct Config {
    key1: Option<String>,
    key2: Option<u64>,
}

let cofig_str = r#"
    key1&v1:1 = "v1" #key1 will be Some("v1") with v1:1, None without v1:1
    key2&v1:1 = "5"  #key2 will be Some(5) with v1:1, None without v1:1
"#;
``` 

### array
  * Rust types: ```Vec<T>```, ```T``` could be premitive types, array or struct.
  * Configuration: All the values in same array must share same Rust data type.
  * Example:
```
#[derive(Deserialize)]
#[serde(crate = "variants::serde")]
struct Config {
    key1: Vec<String>,
    key2: Vec<u64>,
}

let cofig_str = r#"
    key1&v1:1 = <"v0", "v1", "v2"> #with variants
    key1 = <"v0", "v1", "v2">
    key2&v1:1 = <"0", "1", "2"> #with variants
    key2&v1:1 = <0, 1, 2>

"#;
``` 

### struct
  * Rust types: ```struct``` wich fields could be premitive types, array or struct.
  * Configuration: starts with ```[<key>]``` or embeded in ```{}``` in configration.
  * Example:
```
#[derive(Deserialize)]
#[serde(crate = "variants::serde")]
struct Config {
    sub_config: SubConfig,
}

#[derive(Deserialize)]
#[serde(crate = "variants::serde")]
struct SubConfig {
    skey: String,
}

let cofig_str = r#"
    sub_config&v1:1 = {skey = "v1"} #with variants
    sub_config = {skey = "v2"}
"#;

or:

let cofig_str = r#"
    [sub_config&v1:1] #with variants
    skey = "v1"
    [sub_config]
    skey = "v2"
"#;
``` 
---
### [Next: Get started](./Get_started.md)

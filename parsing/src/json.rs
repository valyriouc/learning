use std::collections::HashMap;

// A simple representation of JSON parsing errors
#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken(String),
    InvalidSyntax(String),
    MissingToken(String),
    EmptyInput,
    NotSupported(String)
}

// A simple representation of JSON values
#[derive(Debug, PartialEq)]
pub enum JsonType {
    Object(HashMap<String, JsonType>),
    Array(Vec<JsonType>),
    String(String),
    Number(i64),
    Decimal(f64),
    Boolean(bool)
}

pub fn parse_json(mut input: &str) -> Result<JsonType, ParserError>  {
    if input.trim().is_empty() {
        return Err(ParserError::EmptyInput);
    }

    input = &input.trim_start();
    
    match input.chars().nth(0).unwrap() {
        '{' => {
            // Parse JSON object
            match parse_object(&input) {
                Ok(obj) => Ok(JsonType::Object(obj.0)),
                Err(e) => Err(e)
            }
        },  
        '[' => {
            // Parse JSON array
            match parse_array(&input) {
                Ok(arr) => Ok(JsonType::Array(arr.0)),
                Err(e) => Err(e)
            }
        },
        _ => return Err(ParserError::UnexpectedToken(format!("Unexpected token: {}", input.chars().nth(0).unwrap())))
    }
}

fn parse_boolean(input: &str) -> Result<(bool, &str), ParserError> {
    match input.chars().nth(0) {
        Some('t') => {
            if input.len() < 4 {
                return Err(ParserError::InvalidSyntax(format!("Invalid boolean: {}", input)));
            }

            match &input[..4] {
                "true" => return Ok((true, &input[4..])),
                _ => return Err(ParserError::InvalidSyntax(format!("Invalid boolean: {}", input)))
            };
        },
        Some('f') => {
            if input.len() < 5 {
                return Err(ParserError::InvalidSyntax(format!("Invalid boolean: {}", input)));
            }

            match &input[..5] {
                "false" => return Ok((false, &input[5..])),
                _ => return Err(ParserError::InvalidSyntax(format!("Invalid boolean: {}", input)))
            };
        },
        _ => return Err(ParserError::UnexpectedToken(format!("Expected boolean, found: {}", input.chars().nth(0).unwrap_or(' '))))
    }
}

fn parse_string(input: &str) -> Result<(String, &str), ParserError> {
    if !input.starts_with('"') {
        return Err(ParserError::InvalidSyntax(format!("String must start with a quote: {}", input)));
    }

    let end_quote_pos = input[1..].find('"');
    match end_quote_pos {
        Some(pos) =>{
            let value = &input[1..pos+1].to_string();
            Ok((value.to_string(), &input[pos+2..]))
        } 
        None => Err(ParserError::MissingToken("Missing closing quote for string".to_string()))
    }
}

fn parse_number(input: &str) -> Result<(JsonType, &str), ParserError> {
    if input.is_empty() {
        return Err(ParserError::EmptyInput);
    }

    let mut builder = String::new();
    let mut buffer = input;

    loop {
        match buffer.chars().nth(0) {
            Some(c) => {
                match c {
                    '0'..='9' | '-' | '.'=> {
                        builder.push(c);
                        buffer = &buffer[1..];
                    },
                    _ =>  {
                        if builder.is_empty() {
                            return Err(ParserError::InvalidSyntax(format!("Invalid number: {}", input)));
                        } 
                        
                        if builder.contains('.') {
                            match builder.parse::<f64>() {
                                Ok(num) => return Ok((JsonType::Decimal(num), buffer)),
                                Err(_) => return Err(ParserError::InvalidSyntax(format!("Invalid number: {}", builder)))
                            }
                        }
                        else {
                            match builder.parse::<i64>() {
                                Ok(num) => return Ok((JsonType::Number(num), buffer)),
                                Err(_) => return Err(ParserError::InvalidSyntax(format!("Invalid number: {}", builder)))
                            }
                        }
                    }
                }
            }
            None => return Err(ParserError::EmptyInput),    
        }
    }
}

fn parse_array(mut input: &str) -> Result<(Vec<JsonType>, &str), ParserError> {
    let mut result = Vec::<JsonType>::new();
    
    if (input.chars().nth(0).unwrap()) != '[' {
        return Err(ParserError::InvalidSyntax("Array must start with '['".to_string()));
    }

    input = &input[1..].trim_start();

    loop {
        // todo: skip whitespaces and so on 
    
        if input.starts_with(']') {
            break;
        }

        match input.chars().nth(0).unwrap() {
            '{' => {
                match parse_object(&input) {
                    Ok(obj) => {
                        input = obj.1.trim_start();
                        result.push(JsonType::Object(obj.0))
                    },
                    Err(e) => return Err(e)
                }
            },
            '[' => {
                return Err(ParserError::NotSupported("Nested arrays not supported yet".to_string()));
            }
            '"' => {
                match parse_string(input) {
                    Ok(s) => {
                        input = s.1.trim_start();
                        result.push(JsonType::String(s.0))
                    },
                    Err(e) => return Err(e)
                }
            },
            't' | 'f' => {
                match parse_boolean(input) {
                    Ok(b) => {
                        input = b.1.trim_start();
                        result.push(JsonType::Boolean(b.0))
                    },
                    Err(e) => return Err(e)
                }
            },
            '0'..='9' => {
                match parse_number(input) {
                    Ok(n) => {
                        input = n.1.trim_start();
                        result.push(n.0)
                    },
                    Err(e) => return Err(e)
                }
            },
            _ => return Err(ParserError::UnexpectedToken(format!("Unexpected token in array: {}", input.chars().nth(0).unwrap())))
        }

        if input.chars().nth(0).unwrap() == ',' {
            // skip comma
            input = &input[1..].trim_start();
        }
        else if input.chars().nth(0).unwrap() == ']' {
            input = &input[1..].trim_start();
            break;
        }
        else {
            return Err(ParserError::UnexpectedToken(format!("Expected ',' or ']' in array, found: {}", input.chars().nth(0).unwrap())));
        }
    }

    Ok((result, &input))
}

fn parse_object(mut input: &str) -> Result<(HashMap<String, JsonType>, &str), ParserError> {
    let mut result = HashMap::new();
    
    if input.chars().nth(0).unwrap() != '{' {
        return Err(ParserError::InvalidSyntax("Object must start with '{'".to_string()));
    }
    
    input = &input[1..].trim_start();

    loop {
        // Parse each key-value pair
        if (input.chars().nth(0).unwrap()) == '}' {
            return Ok((result, &input[1..])); // Empty object
        }

        match parse_string(&input) {
            Ok(key) => {
                // Expect a colon
                input = key.1;

                if input.chars().nth(0).unwrap() != ':' {
                    return Err(ParserError::MissingToken("Expected ':' after key".to_string()));
                }

                input = &input[1..].trim_start();

                let value = if input.chars().nth(0).unwrap() == '{' {
                    match parse_object(&input) {
                        Ok(obj) => {
                            input = obj.1;
                            JsonType::Object(obj.0)
                        },
                        Err(e) => return Err(e)
                    }
                } else if input.chars().nth(0).unwrap() == '[' {
                    match parse_array(&input) {
                        Ok(arr) => {
                            input = arr.1;
                            JsonType::Array(arr.0)
                        },
                        Err(e) => return Err(e)
                    }
                } else if input.chars().nth(0).unwrap() == '"' {
                    match parse_string(input) {
                        Ok(s) => {
                            input = s.1;
                            JsonType::String(s.0)
                        },
                        Err(e) => return Err(e)
                    }
                } else if input.chars().nth(0).unwrap() == 't' || input.chars().nth(0).unwrap() == 'f' {
                    match parse_boolean(input) {
                        Ok(b) => {
                            input = b.1;
                            JsonType::Boolean(b.0)
                        },
                        Err(e) => return Err(e)
                    }
                } else if input.chars().nth(0).unwrap().is_digit(10) || input.chars().nth(0).unwrap() == '-' {
                    match parse_number(input) {
                        Ok(n) => {
                            input = n.1;
                            n.0
                        },
                        Err(e) => return Err(e)
                    }
                } else {
                    return Err(ParserError::UnexpectedToken(format!("Unexpected token in object value: {}", input.chars().nth(0).unwrap())));
                };

                result.insert(key.0, value);
                input = input.trim_start();

                // Check for comma or end of object
                if input.chars().nth(0).unwrap() == ',' {
                    // Move past the comma
                    input = &input[1..].trim_start();
                } else if input.chars().nth(0).unwrap() == '}' {
                    input = &input[1..].trim_start();
                    break; // End of object
                } else {
                    return Err(ParserError::UnexpectedToken(format!("Expected ',' or '}}' in object, found: {}", input.chars().nth(0).unwrap())));
                }
            },
            Err(e) => return Err(e)
        }
    }

    Ok((result, &input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_json_object_with_boolean_property_true() {
        let result = parse_json(r#"{"key": true}"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Object(map) => {
                let actual = map.get("key").unwrap();
                let expected = JsonType::Boolean(true);
                assert_eq!(actual, &expected);
            },
            _ => panic!("Expected JSON object")
        }
    }

    #[test]
    fn read_json_object_with_boolean_property_false() {
        let result = parse_json(r#"{"key": false}"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Object(map) => {
                let actual = map.get("key").unwrap();
                let expected = JsonType::Boolean(false);
                assert_eq!(actual, &expected);
            },
            _ => panic!("Expected JSON object")
        }
    }

    #[test]
    fn read_json_object_with_multiple_properties() {
        let result = parse_json(r#"{"key1": true, "key2": 123, "key3": "value"}"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Object(map) => {
                let actual1 = map.get("key1").unwrap();
                let expected1 = JsonType::Boolean(true);
                assert_eq!(actual1, &expected1);

                let actual2 = map.get("key2").unwrap();
                let expected2 = JsonType::Number(123);
                assert_eq!(actual2, &expected2);

                let actual3 = map.get("key3").unwrap();
                let expected3 = JsonType::String("value".to_string());
                assert_eq!(actual3, &expected3);
            },
            _ => panic!("Expected JSON object")
        }
    }

    #[test]
    fn read_json_object_with_multiple_properties_containing_array() {
        let result = parse_json(r#"{"key1": true, "key2": [1, 2, 3], "key3": "value"}"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Object(map) => {
                let actual1 = map.get("key1").unwrap();
                let expected1 = JsonType::Boolean(true);
                assert_eq!(actual1, &expected1);

                let actual2 = map.get("key2").unwrap();
                let expected2 = JsonType::Array(vec![
                    JsonType::Number(1),
                    JsonType::Number(2),
                    JsonType::Number(3)
                ]);
                assert_eq!(actual2, &expected2);

                let actual3 = map.get("key3").unwrap();
                let expected3 = JsonType::String("value".to_string());
                assert_eq!(actual3, &expected3);
            },
            _ => panic!("Expected JSON object")
        }
    }

    #[test]
    fn read_json_object_with_multiple_properties_containing_object() {
        let result = parse_json(r#"{"key1": true, "key2": {"subkey": "subvalue"}, "key3": "value"}"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Object(map) => {
                let actual1 = map.get("key1").unwrap();
                let expected1 = JsonType::Boolean(true);
                assert_eq!(actual1, &expected1);

                let actual2 = map.get("key2").unwrap();
                let mut sub_map = HashMap::new();
                sub_map.insert("subkey".to_string(), JsonType::String("subvalue".to_string()));
                let expected2 = JsonType::Object(sub_map);
                assert_eq!(actual2, &expected2);

                let actual3 = map.get("key3").unwrap();
                let expected3 = JsonType::String("value".to_string());
                assert_eq!(actual3, &expected3);
            },
            _ => panic!("Expected JSON object")
        }
    }

    #[test]
    fn read_json_object_with_number_property() {
        let result = parse_json(r#"{"key": 123}"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Object(map) => {
                let actual = map.get("key").unwrap();
                let expected = JsonType::Number(123);
                assert_eq!(actual, &expected);
            },
            _ => panic!("Expected JSON object")
        }
    }

    #[test]
    fn read_json_object_with_array_property() {
        let result = parse_json(r#"{"key": [1, 2, 3]}"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Object(map) => {
                let actual = map.get("key").unwrap();
                let expected = JsonType::Array(vec![
                    JsonType::Number(1),
                    JsonType::Number(2),
                    JsonType::Number(3)
                ]);
                assert_eq!(actual, &expected);
            },
            _ => panic!("Expected JSON object")
        }
    }

    #[test]
    fn read_json_object_with_string_property() {
        let result = parse_json(r#"{"key": "value"}"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Object(map) => {
                let actual = map.get("key").unwrap();
                let expected = JsonType::String("value".to_string());
                assert_eq!(actual, &expected);
            },
            _ => panic!("Expected JSON object")
        }
    }

    #[test]
    fn read_json_object_with_no_property() {
        let result = parse_json(r#"{}"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Object(map) => {
                assert!(map.is_empty());
            },
            _ => panic!("Expected JSON object")
        }
    }

    #[test]
    fn read_json_object_with_real_world() {
        let json = r#"
        {
    "_id": "68d7cd3e0c429cb0c5dde37b",
    "index": 2,
    "guid": "36f8338a-f478-409e-9c62-dd4a996bd708",
    "isActive": true,
    "balance": "$3,324.92",
    "picture": "http://placehold.it/32x32",
    "age": 22,
    "eyeColor": "brown",
    "name": "Myrtle Terrell",
    "gender": "female",
    "company": "ZENSUS",
    "email": "myrtleterrell@zensus.com",
    "phone": "+1 (923) 486-3743",
    "address": "511 Nova Court, Tetherow, Indiana, 5807",
    "about": "Non cillum adipisicing consequat sunt tempor pariatur occaecat sint laborum sit. Exercitation dolore duis occaecat proident elit enim. Nostrud aliquip incididunt reprehenderit ipsum et excepteur exercitation.\r\n",
    "registered": "2020-05-02T02:37:23 -02:00",
    "latitude": 56.2324,
    "longitude": -140.453,
    "tags": [
      "reprehenderit",
      "laboris",
      "duis",
      "aute",
      "velit",
      "dolore",
      "in"
    ],
    "friends": [
      {
        "id": 0,
        "name": "Isabella Lawrence"
      },
      {
        "id": 1,
        "name": "Eve Ayala"
      },
      {
        "id": 2,
        "name": "Marion Lucas"
      }
    ],
    "greeting": "Hello, Myrtle Terrell! You have 2 unread messages.",
    "favoriteFruit": "apple"
  }"#;
        let result = parse_json(json);
        match result {
            Ok(_) => {},
            Err(e) => panic!("Failed to parse real-world JSON: {:?}", e)
        }
        assert!(result.is_ok());

    }

    #[test]
    fn read_json_array_with_numbers() {
        let result = parse_json(r#"[1, 2, 3]"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Array(arr) => {
                let expected = vec![
                    JsonType::Number(1),
                    JsonType::Number(2),
                    JsonType::Number(3)
                ];
                assert_eq!(arr, expected);
            },
            _ => panic!("Expected JSON array")
        }
    }

    #[test]
    fn read_json_array_with_strings() {
        let result = parse_json(r#"["one", "two", "three"]"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Array(arr) => {
                let expected = vec![
                    JsonType::String("one".to_string()),
                    JsonType::String("two".to_string()),
                    JsonType::String("three".to_string())
                ];
                assert_eq!(arr, expected);
            },
            _ => panic!("Expected JSON array")
        }
    }

    #[test]
    fn read_json_array_with_booleans() {
        let result = parse_json(r#"[true, false, true]"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Array(arr) => {
                let expected = vec![
                    JsonType::Boolean(true),
                    JsonType::Boolean(false),
                    JsonType::Boolean(true)
                ];
                assert_eq!(arr, expected);
            },
            _ => panic!("Expected JSON array")
        }
    }

    #[test]
    fn read_json_array_with_objects() {
        let result = parse_json(r#"[{"key1": "value1"}, {"key2": "value2"}]"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Array(arr) => {
                let mut obj1 = HashMap::new();
                obj1.insert("key1".to_string(), JsonType::String("value1".to_string()));
                let mut obj2 = HashMap::new();
                obj2.insert("key2".to_string(), JsonType::String("value2".to_string()));
                let expected = vec![
                    JsonType::Object(obj1),
                    JsonType::Object(obj2)
                ];
                assert_eq!(arr, expected);
            },
            _ => panic!("Expected JSON array")
        }
    }

    #[test]
    fn read_json_array_with_no_elements() {
        let result = parse_json(r#"[]"#);
        assert!(result.is_ok());
        let json = result.unwrap();
        match json {
            JsonType::Array(arr) => {
                assert!(arr.is_empty());
            },
            _ => panic!("Expected JSON array")
        }
    }

    #[test]
    fn read_json_array_real_world() {
        let json = r#"
        [
    {
      "message": "Hello, Garnet! Your order number is: #99",
      "phoneNumber": "1-617-301-1258",
      "phoneVariation": "+90 371 278 10 10",
      "status": "disabled",
      "name": {
        "first": "Garett",
        "middle": "Taylor",
        "last": "Koss"
      },
      "username": "Garett-Koss",
      "password": "AnObBJpQvgTqy29",
      "emails": [
        "Reece51@example.com",
        "Armando_Runolfsdottir6@example.com"
      ],
      "location": {
        "street": "212 Orchard Road",
        "city": "Millcreek",
        "state": "Missouri",
        "country": "New Caledonia",
        "zip": "66298",
        "coordinates": {
          "latitude": "19.3044",
          "longitude": "-174.344"
        }
      },
      "website": "https://angelic-raincoat.org",
      "domain": "organic-identity.biz",
      "job": {
        "title": "Principal Usability Supervisor",
        "descriptor": "Principal",
        "area": "Markets",
        "type": "Agent",
        "company": "Rodriguez - Hermann"
      },
      "creditCard": {
        "number": "3685-394036-2106",
        "cvv": "352",
        "issuer": "visa"
      },
      "uuid": "83cb6169-97f5-4c75-94af-26c0e94ce892",
      "objectId": "68d7d40fa9ae9d0823e3d8d1"
    },
    {
      "message": "Hello, Boyd! Your order number is: #49",
      "phoneNumber": "1-374-377-2775 x755",
      "phoneVariation": "+90 302 338 10 22",
      "status": "disabled",
      "name": {
        "first": "Delta",
        "middle": "Angel",
        "last": "Bashirian"
      },
      "username": "Delta-Bashirian",
      "password": "Iix0q_SNu603kKg",
      "emails": [
        "Tristian.Lowe61@gmail.com",
        "Ned.Crona@example.com"
      ],
      "location": {
        "street": "8417 Jefferson Street",
        "city": "East Brenda",
        "state": "New Jersey",
        "country": "Guyana",
        "zip": "67190-9477",
        "coordinates": {
          "latitude": "-19.3501",
          "longitude": "38.6842"
        }
      },
      "website": "https://aching-membership.org/",
      "domain": "frayed-gang.net",
      "job": {
        "title": "Dynamic Optimization Analyst",
        "descriptor": "National",
        "area": "Solutions",
        "type": "Liaison",
        "company": "Carter - Little"
      },
      "creditCard": {
        "number": "50182196861164844478",
        "cvv": "739",
        "issuer": "jcb"
      },
      "uuid": "7b98c76f-945a-4b7d-9692-820bf8022053",
      "objectId": "68d7d40fa9ae9d0823e3d8d2"
    }
  ]
        "#;

        let result = parse_json(json);
        assert!(result.is_ok());
    }
}
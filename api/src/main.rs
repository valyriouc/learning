use parsing::{parse_json, JsonType, FromJson};

fn main() {

    let json_str = r#"
    {
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "courses": ["Math", "Science", "History"],
        "address": {
            "street": "123 Main St",
            "city": "Anytown",
            "zip": "12345"
        }
    }
    "#;

    let parse_result = parse_json(json_str);
    match parse_result {
        Ok(json) => {
            let person = Person::from_json(&json);
            println!("Name: {}", person.name);
            println!("Age: {}", person.age);
            println!("Is Student: {}", person.is_student);
            println!("Courses: {:?}", person.courses);
            println!("Address: {}, {}, {}", person.address.street, person.address.city, person.address.zip);
        },
        Err(e) => {
            println!("Failed to parse JSON: {:?}", e);
        }
    }
}

struct Person {
    name: String,
    age: i32,
    is_student: bool,
    courses: Vec<String>,
    address: Address,
}

impl FromJson for Person {
    fn from_json(json: &JsonType) -> Self {
        match json {
            JsonType::Object(obj) => {
                let name = if let Some(JsonType::String(s)) = obj.get("name") {
                    s.clone()
                } else {
                    "".to_string()
                };

                let age = if let Some(JsonType::Number(n)) = obj.get("age") {
                    *n as i32
                } else {
                    0
                };

                let is_student = if let Some(JsonType::Boolean(b)) = obj.get("is_student") {
                    *b
                } else {
                    false
                };

                let courses = if let Some(JsonType::Array(arr)) = obj.get("courses") {
                    arr.iter().filter_map(|item| {
                        if let JsonType::String(s) = item {
                            Some(s.clone())
                        } else {
                            None
                        }
                    }).collect()
                } else {
                    vec![]
                };

                let address = if let Some(addr_json) = obj.get("address") {
                    Address::from_json(addr_json.clone())
                } else {
                    Address {
                        street: "".to_string(),
                        city: "".to_string(),
                        zip: "".to_string(),
                    }
                };

                Person {
                    name,
                    age,
                    is_student,
                    courses,
                    address,
                }
            },
            _ => panic!("Expected a JSON object"),
        }
    }
}

struct Address {
    street: String,
    city: String,
    zip: String,
}

impl FromJson for Address {
    fn from_json(json: &JsonType) -> Self {
        match json {
            JsonType::Object(obj) => {
                let street = if let Some(JsonType::String(s)) = obj.get("street") {
                    s.clone()
                } else {
                    "".to_string()
                };

                let city = if let Some(JsonType::String(s)) = obj.get("city") {
                    s.clone()
                } else {
                    "".to_string()
                };

                let zip = if let Some(JsonType::String(s)) = obj.get("zip") {
                    s.clone()
                } else {
                    "".to_string()
                };

                Address {
                    street,
                    city,
                    zip,
                }
            },
            _ => panic!("Expected a JSON object"),
        }
    }
}
use lite_json::json::{JsonValue, NumberValue};
use lite_json::json_parser::parse_json;
use lite_json::json_parser::parse_json_with_options;
use lite_json::Serialize;
use lite_parser::parser::ParserOptions;

fn main() {
    if false {
        // Creating JSON
        // We will create a bunch of elements that we will put into a JSON Object.
        let mut object_elements = vec![];

        // Create a boolean value and add it to our vector.
        let boolean_value = true;
        let object_key = "boolean".chars().collect();
        object_elements.push((object_key, JsonValue::Boolean(boolean_value)));

        // Create an array value and add it to our vector.
        let array_value = vec![
            JsonValue::Boolean(true),
            JsonValue::Boolean(false),
            JsonValue::Boolean(true),
        ];
        let object_key = "array".chars().collect();
        object_elements.push((object_key, JsonValue::Array(array_value)));

        // Create a string value and add it to our vector.
        let string_value = "Hello World!".chars().collect();
        let object_key = "string".chars().collect();
        object_elements.push((object_key, JsonValue::String(string_value)));

        // Create a number value and add it to our vector.
        let number_value = NumberValue {
            integer: 1234,
            fraction: 0,
            fraction_length: 0,
            exponent: 0,
        };
        let object_key = "number".chars().collect();
        object_elements.push((object_key, JsonValue::Number(number_value)));

        // Create a null value and add it to our vector.
        let object_key = "null".chars().collect();
        object_elements.push((object_key, JsonValue::Null));

        // Create the object value from the vector of elements.
        let object_value = JsonValue::Object(object_elements);

        // Convert the object to a JSON string.
        let json = object_value.format(4);
        let json_output = std::str::from_utf8(&json).unwrap();

        println!("{}", json_output);
    }

    if false {
        // Parsing JSON
        // This is the JSON string we will use.
        let json_string = r#"
            {
                "boolean": true,
                "array":
                [
                    true,
                    false,
                    true
                ],
                "string": "Hello World!",
                "number": 1234,
                "null": null
            }
        "#;

        // Parse the JSON and print the resulting lite-json structure.
        let json_data = parse_json(json_string).expect("Invalid JSON specified!");
        println!("{:?}", json_data);
    }

    if true {
        // Parsing JSON with Options
        // This is the JSON string we will use.
        let json_string = r#"
            {
                "boolean": true,
                "array":
                [
                    true,
                    false,
                    true
                ],
                "string": "Hello World!",
                "number": 1234,
                "null": null
            }
        "#;

        let parser_options = ParserOptions {
            max_nest_level: Some(2),
        };

        // Parse the JSON and print the resulting lite-json structure.
        let json_data =
            parse_json_with_options(json_string, parser_options).expect("Invalid JSON specified!");
        println!("{:?}", json_data);
    }
}

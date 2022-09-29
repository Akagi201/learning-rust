use prost::Message;
use prost_live::pb::*;

fn main() {
    if false {
        let person = Person::default();
        let v1 = person.encode_to_vec();
        let v2 = person.encode_length_delimited_to_vec();

        println!("{person:?}, {v1:?}, {v2:?}");
    }
    {
        let phones = vec![PhoneNumber::new("1234567890", PhoneType::Mobile)];
        let person = Person::new("Bob".into(), 1, "abc@gmail.com", phones);
        let v1 = person.encode_to_vec();
        let person1 = Person::decode(v1.as_ref()).unwrap();
        assert_eq!(person, person1);

        let json = serde_json::to_string_pretty(&person1).unwrap();

        // println!("{person:?}, {v1:?}(len: {})", v1.len());
        println!("{}", json);
    }
}

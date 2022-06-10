use std::io::Read;

use crate::args;

use anyhow::Result;
use flate2::bufread::GzDecoder;
use openssl::base64;

pub fn execute(args: args::ArgsElephant) -> Result<()> {
    let data: &str = match args.number {
        1 => {
            "H4sIAJVWBU4CA21RMQ7DIBDbeYWrDgQJ7rZ+IA/IB05l69alcx5fc0ASVXUk4jOO\
              7yAAUWtorygwJ4hlMii0YkJKKRKGvsMsiykl1SalvrMD1gUXyXRkGZPx5OPft81K\
              tNAiAjyGjYO47h1JjizPkJrCWbK/4C+uLkT7bzpGc7CT9bmOzNSW5WLSO5vexjmH\
              ZL9JFFZeAa0a2+lKjL2anpYfV+0Zx9LJ+/MC8nRayuDlSNy2rfAPibOzsiWHL0jL\
              SsjFAQAA"
        } // Elephant 2
        2 => {
            "H4sICPQnm2IAA3RvcnRvaXNlAB2NsQlDQQxD+z+FO9uFdKNkAcPr0qVJneFzdwKB\
              9BAo4gpb0raf250T8FsvQ1+0wBmssJI5qOAwizmp4+5qT2goYuhhw8/3XfugpbLp\
              lPL5A+WWG6h4AAAA"
        }
        _ => {
            "H4sIAL3qBE4CAyWLwQ3AMAgD/0xh5UPzYiFUMgjq7LUJsk7yIQNAQTAikFUDnqkr\
              OQFOUm0Wd9pHCi13ONjBpVdqcWx+EdXVX4vXvGv5cgztB9+fJxZ7AAAA"
        } // Elephant 1
    };

    let bytes = base64::decode_block(data)?; // Cannot fails

    let mut elephant = String::new();
    let mut decoder = GzDecoder::new(bytes.as_slice());
    decoder.read_to_string(&mut elephant)?; // Cannot fails

    println!("{}", elephant);

    Ok(())
}

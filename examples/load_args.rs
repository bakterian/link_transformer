extern crate link_transformer;
use std::{env, io::{ErrorKind, Error}};

fn main() -> Result<(), std::io::Error>
{
    let link_to_transform = env::args().nth(1).expect("Missing a input argument, provide a link to transform !!!");
    println!("Attempting to tranform the following link: {}", link_to_transform);

    let recipe_path_str = env::args().nth(2).expect("Missing a input argument, provide a link to translation recipe !!!");
    let recipes_path = std::path::PathBuf::from(&recipe_path_str);
    if !recipes_path.exists() {
        let error = Error::new(ErrorKind::NotFound, "recipe file not found!");
        return Err(error);
    }
    println!("Passing over the following translation recipe: \n{}", recipe_path_str);

    let transformed_link: Option<String> = link_transformer::convert(&recipe_path_str[..], &link_to_transform[..]);
    
    match transformed_link {
        None => println!("Transformation failed. Link base-url is unkown."),
        Some(s) => println!("Tranformetion restult : {}", &s[..])
    };

    Ok(())
}

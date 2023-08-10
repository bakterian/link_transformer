extern crate link_transformer;
use std::env;

static RECIPE_FILE_LOCATION: &str = r"examples\recipes.json";  

fn main() -> Result<(), std::io::Error>
{
    let link_to_transform = r"http://goo.gl/maps";
    println!("Attempting to tranform the following link: {}", link_to_transform);

    let mut recipes_path = env::current_exe()?;
    recipes_path.pop();
    recipes_path.pop();
    recipes_path.pop();
    recipes_path.pop();
    recipes_path = recipes_path.join(RECIPE_FILE_LOCATION);
    
    let recipe_path_str = recipes_path.to_str().unwrap();

    println!("Passing over the following translation recipe: {}", recipe_path_str);

    let transformed_link: Option<String> = link_transformer::convert(recipe_path_str, link_to_transform);
    
    match transformed_link {
        None => println!("Transformation failed. Link base-url is unkown."),
        Some(s) => println!("Tranformetion was successfull, restult : {}", &s[..])
    };

    Ok(())
}

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct TransformRecipe
{
    input_url: String,
    output_url: String,
    hidden_folders: Vec<String>
}

#[derive(Deserialize)]
struct RecipesCollection
{
    pub collection:  Vec<TransformRecipe>
}


fn get_recipes(recipes_path: &str) -> RecipesCollection
{
    let file_path = std::path::PathBuf::from(recipes_path);

    let recipes_file_contents = std::fs::read_to_string(&file_path).unwrap();

    serde_json::from_str::<RecipesCollection>(&recipes_file_contents).unwrap()
}

fn get_link(recipes_collection: RecipesCollection, link_to_transform: &str) -> Option<String>
{
    let mut transformed_link: Option<String> = None;

    for recipe in recipes_collection.collection
    {
        if link_to_transform[..].contains(recipe.input_url.as_str())
        {
            let mut link = link_to_transform.to_owned();
            link = link.replace(recipe.input_url.as_str(), "");

            for hidden_folder in recipe.hidden_folders
            {                
                match link.find(hidden_folder.as_str()) {
                    None => (),
                    Some(i)  => link.insert(i+hidden_folder.len(), '$')
                }                     
            }
        
            link.insert_str(0, recipe.output_url.as_str());
            transformed_link = Some(link); 

            break;
        }
    }
    transformed_link
}

pub fn convert(recipes_path: &str, link_to_transform: &str) -> Option<String>
{
    let recipes_collection =  get_recipes(recipes_path);
    
    let transformed_link = get_link(recipes_collection, link_to_transform);

    if transformed_link.is_none() {
        println!("Transformation failed. Link base-url is unkown.")
    }

    transformed_link
}

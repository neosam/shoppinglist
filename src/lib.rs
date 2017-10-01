extern crate uuid;

use std::collections::HashMap;
use std::vec::Vec;
use std::marker::PhantomData;

#[macro_use] extern crate serde_derive;
//extern crate serde;

use uuid::Uuid;

pub struct Key<T> {pub key: Uuid, phantom: PhantomData<T> }
impl<T> Key<T> {
    pub fn new(uuid: Uuid) -> Key<T> {
        Key { key: uuid, phantom: PhantomData }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub key: Uuid,
    pub name: String
}

impl Ingredient {
    fn new<S: ToString>(name: S) -> Ingredient {
        let name = name.to_string();
        Ingredient {
            key: Uuid::new_v4(),
            name
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub key: Uuid,
    pub name: String,
    pub ingredients: Vec<(Uuid, f32)>
}

impl Recipe {
    pub fn new<S: ToString>(name: S) -> Recipe {
        Recipe {
            key: Uuid::new_v4(),
            name: name.to_string(),
            ingredients: Vec::new()
        }
    }

    pub fn add_ingredient(&mut self, ingredient_key: Key<Ingredient>, amount: f32) {
        self.ingredients.push((ingredient_key.key, amount));
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShoppingListItem {
    pub key: Uuid,
    pub ingredient_key: Uuid,
    pub amount: f32,
    pub recipe_key: Option<Uuid>
}

impl ShoppingListItem {
    pub fn new<O: Into<Option<Key<Recipe>>>>(ingredient_key: Key<Ingredient>, recipe_key: O, amount: f32) -> ShoppingListItem {
        ShoppingListItem {
            key: Uuid::new_v4(),
            ingredient_key: ingredient_key.key,
            amount,
            recipe_key: recipe_key.into().map(|x| x.key )
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShoppingList {
    pub ingredients: HashMap<Uuid, Ingredient>,
    pub recipes: HashMap<Uuid, Recipe>,
    pub shopping_list: Vec<ShoppingListItem>
}

impl ShoppingList {
    pub fn new() -> ShoppingList {
        ShoppingList {
            ingredients: HashMap::new(),
            recipes: HashMap::new(),
            shopping_list: Vec::new()
        }
    }

    pub fn insert_ingredient<S: ToString>(&mut self, name: S) -> Key<Ingredient> {
        let ingredient = Ingredient::new(name);
        let key = ingredient.key.clone();
        self.ingredients.insert(ingredient.key.clone(), ingredient);
        Key::new(key)
    }

    pub fn ingredient_iter<'a>(&'a self) -> Box<Iterator<Item=&'a Ingredient> + 'a> {
        Box::new(self.ingredients.values())
    }

    pub fn ingredient(&self, key: Key<Ingredient>) -> &Ingredient {
        self.ingredients.get(&key.key).unwrap()
    }

    pub fn generate_recipe<S: ToString>(&mut self, name: S) -> &mut Recipe {
        let recipe = Recipe::new(name);
        let recipe_key = recipe.key.clone();
        self.recipes.insert(recipe_key.clone(), recipe);
        let recipe = self.recipes.get_mut(&recipe_key).unwrap();
        return recipe;
    }

    pub fn recipe_iter<'a>(&'a self) -> Box<Iterator<Item=&'a Recipe> + 'a> {
        Box::new(self.recipes.values())
    }

    pub fn recipe(&self, key: Key<Recipe>) -> &Recipe {
        self.recipes.get(&key.key).unwrap()
    }

    pub fn recipe_mut(&mut self, key: Key<Recipe>) -> &mut Recipe {
        self.recipes.get_mut(&key.key).unwrap()
    }

    pub fn shoppinglist_iter<'a>(&'a self) -> Box<Iterator<Item=&'a ShoppingListItem> + 'a> {
        Box::new(self.shopping_list.iter())
    }

    pub fn insert_shoppinglist(&mut self, ingredient_key: Key<Ingredient>, amount: f32) -> Key<ShoppingListItem> {
        let shopping_list_item = ShoppingListItem::new(
            ingredient_key,
            None,
            amount);
        let key = shopping_list_item.key.clone();
        self.shopping_list.push(shopping_list_item);
        Key::new(key)

    }

    pub fn add_recipe_to_list(&mut self, recipe_key: Key<Recipe>, multiplier: f32) {
        let recipe_id = recipe_key.key.clone();
        let ingredients = self.recipe(recipe_key).ingredients.clone();
        for (ingredient_key, amount) in ingredients {
            self.shopping_list.push(ShoppingListItem::new(Key::new(ingredient_key),
                                                          Some(Key::new(recipe_id.clone())),
                                                          amount * multiplier));
        }
    }
}

use reqwest::blocking::{get, Response};
use scraper::{Html, Selector};

fn main() {
    // Collect the argument from cargo run {argument}
    let args: Vec<String> = std::env::args().collect();

    // Simply verify whether that an recipe has been added
    println!("args: {:?}", args[1]);
    if args.len() < 2 {
        eprintln!("Usage: cargo run <search_query.");
        std::process::exit(1);
    }

    if let Ok(url) = get_recipe_url(args[1].to_string()) {
        let second_url: String = url.clone();
        if let Ok(items) = get_recipe_items(url) {
            println!("Ingredients: ");
            for val in items {
                println!("{:?}", val);
            }
        }

        if let Ok(description) = get_recipe_description(second_url) {
            println!("Description: ");
            for val in description {
                println!("{:?}", val);
            }
        }
    }
}




// Website specific functions
fn get_recipe_url(args: String) -> Result<String, Box<dyn std::error::Error>> {
    // Construct the url
    let url: String = format!("https://www.arla.dk/opskrifter/?search={:?}", args);

    // Get the html file of the website
    let recipe_checker: Response = get(url)?;

    // parse it to something we can work with
    let html = Html::parse_fragment(&recipe_checker.text()?);

    // Use a CSS selector to find the
    let recipe_selector =
        Selector::parse("a.u-flex.c-card__image-wrap.c-card__image-wrap--large").unwrap();
    let recipe_element: Option<_> = html.select(&recipe_selector).next();

    // Check if the element was found
    let recipe_element = match recipe_element {
        Some(element) => element,
        None => return Err("Recipe element not found".into()),
    };

    // Extract the href attribute
    let href: String =
        "https://www.arla.dk".to_owned() + recipe_element.value().attr("href").unwrap_or_default();
    println!("Recipe Href: {:?}", href);

    Ok(href)
}

fn get_recipe_items(url: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let food_url: String = url;

    let food_checker: Response = get(food_url)?;

    let html = Html::parse_fragment(&food_checker.text()?);

    let div_selector = Selector::parse("div.c-recipe__ingredients-inner").unwrap();

    if let Some(div) = html.select(&div_selector).next() {
        let span_selector = Selector::parse("table span").unwrap();

        let items: Vec<String> = div
            .select(&span_selector)
            .map(|span| span.text().collect())
            .collect();

        let clean_vector: Vec<String> = clean_vector(items);

        return Ok(clean_vector);
    }

    let nothing: Vec<_> = Vec::new();

    Ok(nothing)
}

fn get_recipe_description (url: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let recipe_checker: Response = get(url)?;

    let html = Html::parse_fragment(&recipe_checker.text()?);

    let instruction_selector = Selector::parse(".c-recipe__instructions-step span").unwrap();

    let instructions: Vec<String> = html
        .select(&instruction_selector)
        .map(|element| element.text().collect())
        .collect();

    Ok(instructions)
}




// Dynamic functions
fn clean_vector(v: Vec<String>) -> Vec<String> {
    let cleaned_values: Vec<String> = v
        .iter()
        .map(|ingredient| {
            // Remove leading and trailing whitespaces
            let trimmed = ingredient.trim();

            // Replace all occurrences of '\n' with an empty string
            let without_newline = trimmed.replace("\n", " ");

            // Replace multiple consecutive whitespaces with a single space
            without_newline
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join(" ")
        })
        .collect();

    let final_vec: Vec<String> = check_substring_reuse(cleaned_values);

    final_vec
}

fn check_substring_reuse(v: Vec<String>) -> Vec<String> {
    let mut cleaned_values = v.clone();

    let mut i = 0;
    while i < cleaned_values.len() {
        let mut j = i + 1;
        while j < cleaned_values.len() {
            let shortest = find_shortest(&cleaned_values[i], &cleaned_values[j]);
            let longest;

            if shortest == cleaned_values[i] {
                longest = &cleaned_values[j];
            } else {
                longest = &cleaned_values[i];
            }

            if longest.contains(&shortest) {
                cleaned_values.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }

    cleaned_values
}


fn find_shortest(s1: &String, s2: &String) -> String {
    if s1.len() <= s2.len() {
        return s1.to_string();
    } else {
        return s2.to_string();
    }
}

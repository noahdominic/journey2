// Copyright 2023  Noah Dominic Miranda Silvio
// Licensed under the EUPL v1.2


use std::fs::DirEntry;

#[derive(Debug)]
struct UnexpectedChoiceError;

pub(crate) fn ask_for_location(
) -> Result<(String, crate::journal::Location), Box<dyn std::error::Error>> {
    let location_full_address: String = crate::journal::query::for_string(
        "What's your current location?",
        "[optional address specifiers], <location>",
    )?;

    // Getting location info via API below...
    let api_response_native =
        crate::journal::query::open_meteo::get_location_info(&location_full_address)?;

    // Let user choose which location they want
    let city_info =
        crate::journal::query::user::ask_to_choose_location_from_list(&api_response_native)?;

    println!("\nYou are currently in {}.", city_info);

    Ok((location_full_address, city_info.clone()))
}

pub(crate) fn ask_to_choose_location_from_list(
    api_results: &[crate::journal::Location],
) -> Result<&crate::journal::Location, Box<dyn std::error::Error>> {
    match api_results.len() {
        0 => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "There are no locations in the database with that name.",
        ))),
        1 => Ok(&api_results[0]),
        _ => {
            println!("There are multiple locations found.  Which one is correct?");
            for (i, result) in api_results.iter().enumerate() {
                println!("{}. {}", i + 1, result);
            }
            loop {
                let choice =
                    crate::journal::query::for_usize("Enter the number of your correct location")?;
                if choice > 0 && choice <= api_results.len() {
                    return Ok(&api_results[choice - 1]);
                } else {
                    println!("That doesn't seem to be one of the choices.");
                    continue;
                }
            }
        }
    }
}

pub(crate) fn ask_for_config_file_path() -> Result<String, Box<dyn std::error::Error>> {
    let question = "Where do you want to put config.toml?";
    let home_dir = dirs::home_dir().ok_or(crate::journal::file::FileError::HomeDirNotFound)?;
    let binding = home_dir.join(env!("CARGO_PKG_NAME"));
    let hint = binding
        .to_str()
        .ok_or(crate::journal::file::FileError::HomeDirNotFound)?;
    let config_file_path = crate::journal::query::for_string(question, hint)?;
    Ok(config_file_path)
}

pub(crate) fn ask_for_text_editor_multchoice() -> Result<String, Box<dyn std::error::Error>> {
    let choice = crate::journal::query::for_usize(
        r#"Which text editor would you like to use?
    1. Vim
    2. Emacs
    3. Nano
    4. Pico
    5. Other..."#,
    )?;

    let command = match choice {
        1 => "vim".to_string(),
        2 => "emacs".to_string(),
        3 => "nano".to_string(),
        4 => "pico".to_string(),
        5 => ask_for_text_editor_input()?,
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Uh oh!  Something happened that shouldn't have happened.",
            )));
        }
    };

    Ok(command)
}

fn ask_for_text_editor_input() -> Result<String, Box<dyn std::error::Error>> {
    let command = crate::journal::query::for_string(
        "What commandwould you use to call your favourite text editor?",
        "vim | nano | pico | ...",
    )?;

    Ok(command)
}

pub(crate) fn ask_for_file_to_open(filepaths: &Vec<DirEntry>) -> Result<&DirEntry, Box<dyn std::error::Error>> {
    let mut tries = 5;

    println!("Looks like you have multiple files written today.  (File names are usually formatted as: '[Day].[Hour]-[Minute].txt'.)");
    for (i, filepath) in filepaths.iter().enumerate() {
        println!("{}. {}", i+1, filepath.file_name().to_str().expect("Filepath should have been parse-able."));
    }

    while tries > 0 {
        if let Ok(choice) = crate::journal::query::for_usize("Which file would you want to open?") {
            if choice > 0 && choice <= filepaths.len() {
                return Ok(&filepaths[choice - 1]);
            }
        }

        tries -= 1;
        println!("Invalid index. Please try again.");
    }

    println!("Exceeded maximum failed attempts. Exiting...");
    return Err(Box::try_from(crate::journal::query::PromptError::MaxTriesExceeded).unwrap());
}
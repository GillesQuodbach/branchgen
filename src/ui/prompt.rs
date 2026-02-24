use dialoguer::{Input, Select };

pub fn prompt_pi_it(default_pi: Option<u32>, default_it: Option<u32>) -> (u32, u32) {
    match (default_pi, default_it) {
        (Some(pi), Some(it)) => {
            println!("Current defaults -> PI: {pi}, IT: {it}");

            let choices = vec!["Yes, use these values", "Modify IT only","Modify PI and IT"];

            let selection = Select::new()
                .with_prompt(format!("Use actual PI/IT ? (PI {pi} / IT {it})"))
                .items(&choices)
                .default(0)
                .interact()
                .unwrap();

            match selection {
                0 => (pi, it),
                1 => {
                    let new_it: u32 = Input::new().with_prompt(format!("PI number (current: {it})"))
                        .interact_text()
                        .unwrap();
                    (pi, new_it)
                }
                2 => {
                    let new_pi: u32 = Input::new().with_prompt(format!("PI number (current: {pi})"))
                        .interact_text()
                        .unwrap();

                    let new_it: u32 = Input::new().with_prompt(format!("IT number (current: {it})"))
                        .interact_text()
                        .unwrap();

                    (new_pi, new_it)
                }
                _ => unreachable!()
            }
        }
        _ => {
            println!("No PI/IT defaults found. Please enter them.");
            
            let pi: u32 = Input::new()
                .with_prompt("Pi number")
                .interact().unwrap();
            
            let it: u32 = Input::new()
                .with_prompt("IT number")
                .interact().unwrap();
            
            (pi, it)
        }
    }
}
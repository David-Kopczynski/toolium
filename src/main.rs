use colored::Colorize;
use colors_transform::{Color, Rgb};

mod periodic_table;

fn main() {
    // Define CLI arguments
    let action = std::env::args().nth(1).unwrap_or("help".to_string());

    // Match CLI arguments
    match action.as_str() {
        "help" => {
            println!("Usage: toolium [action]");
            println!("Actions:");
            println!("  help: Display this help message");
            println!("  version [input] [flag]: Generate version naming scheme");
            println!("    --stable: Generate version using stable isotopes");
            println!("    --unstable: Generate version using unstable isotopes");
        }

        "version" => {
            // Get generic input
            let input = std::env::args().nth(2).unwrap_or("".to_string());
            let flag = std::env::args().nth(3).unwrap_or("--stable".to_string());

            // Convert input to index number for periodic table
            let input_sum: usize = input.chars().map(|c| c as usize).sum::<usize>();

            match flag.as_str() {
                "--stable" => {
                    // Get data as stable isotopes
                    let stable_isotopes = periodic_table::ELEMENTS
                        .iter()
                        .filter(|element| element.stable_isotopes.len() > 0)
                        .collect::<Vec<&periodic_table::Element>>();

                    let element = &stable_isotopes[input_sum % stable_isotopes.len()];
                    let stable = element.stable_isotopes[input_sum % element.stable_isotopes.len()];
                    let rgb = Rgb::from_hex_str(element.cpk_hex_color.unwrap_or("000000")).unwrap();

                    println!(
                        "{}",
                        format!("{}-{}", element.name, stable)
                            .to_lowercase()
                            .replace(" ", "-")
                            .truecolor(
                                rgb.get_red() as u8,
                                rgb.get_green() as u8,
                                rgb.get_blue() as u8
                            )
                    );
                }

                "--unstable" => {
                    // Get data as stable isotopes
                    let unstable_isotopes = periodic_table::ELEMENTS
                        .iter()
                        .filter(|element| element.stable_isotopes.len() == 0)
                        .collect::<Vec<&periodic_table::Element>>();

                    let element = &unstable_isotopes[input_sum % unstable_isotopes.len()];
                    let oxidatoin = if element.oxidation_states.len() > 0 {
                        element.oxidation_states[input_sum % element.oxidation_states.len()]
                    } else {
                        0
                    };
                    let rgb = Rgb::from_hex_str(element.cpk_hex_color.unwrap_or("000000")).unwrap();

                    println!(
                        "{}",
                        format!("{}-{}-{}", element.group_block, element.name, oxidatoin)
                            .to_lowercase()
                            .replace(" ", "-")
                            .truecolor(
                                rgb.get_red() as u8,
                                rgb.get_green() as u8,
                                rgb.get_blue() as u8
                            )
                    );
                }

                _ => {
                    println!("Invalid flag: {}", flag);
                    println!("Run 'toolium help' for usage information");
                }
            }
        }

        _ => {
            println!("Invalid action: {}", action);
            println!("Run 'toolium help' for usage information");
        }
    }
}

use std::{env, fs};

const VALID_LICENSES: [&'static str; 16] = ["agpl", "gplv2", "gplv3", "mit", "apache", "wtfpl", "afl", "artistic", "bsd2",
                                            "bsd3", "bsl", "ccbysa", "ccby", "lgplv2.1", "lgplv3", "postgresql"];

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // second arg is license name, third arg is file name
    if args.len() > 1 {
        let license_name: &str = &args[1];

        let mut file_name = "LICENSE";
        if args.len() == 3 {
            file_name = &args[2];
        }

        if VALID_LICENSES.contains(&license_name) {
            let current_dir = env::current_dir()?;
            let output_file = current_dir.join(file_name);

            let templates_path = env::current_exe()?.parent().unwrap().join("templates");
            let template_file = templates_path.join(license_name);

            println!("{:?} {:?} {:?} {:?}", current_dir, output_file, templates_path, template_file);

            fs::copy(template_file, output_file)?;
        } 
        else {
            panic!("Invalid license name!");
        }
    } 
    else {
        panic!("Add a license name at least, dummy! Valid license names are located on the github or README.md")
    }

    Ok(())
}

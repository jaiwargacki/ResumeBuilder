use std::collections::HashMap;
use std::fs::write;
use latexcompile::{LatexCompiler, LatexInput};

use crate::sections::{Resume};

fn create_latex(resume: &Resume) {
    // Get starter text from template
    let mut latex = include_str!("../assets/template.tex").to_string();

    // Personal Info
    latex = latex.replace("{{name}}", &resume.personal_info.name);
    latex = latex.replace("{{email}}", &resume.personal_info.email);
    latex = latex.replace("{{phone}}", &resume.personal_info.phone);


    // Work Experience
    let mut experience = String::new();
    experience.push_str("\\newSection{Professional Experience}\n");

    let mut previous_institution = String::new();
    for entry in &resume.experience {
        if entry.institution != previous_institution {
            experience.push_str("\\newEntity{");
            experience.push_str(&entry.institution);
            experience.push_str("}{");
            experience.push_str(&entry.location);
            experience.push_str("}{");
        } else {
            experience.push_str("\\contEntity{");
        }
        experience.push_str(&entry.title);
        experience.push_str("}{");
        experience.push_str(&entry.start_date);
        experience.push_str(" - ");
        experience.push_str(&entry.end_date);
        experience.push_str("}\n");
        if !entry.description.is_empty() {
            experience.push_str("\\detailListStart\n");
            for bullet in &entry.description {
                experience.push_str("\\detail{");
                experience.push_str(&bullet);
                experience.push_str("}\n");
            }
            experience.push_str("\\detailListEnd\n");
        }
        previous_institution = entry.institution.clone();
    }
    latex = latex.replace("{{work-experience}}", &experience);

    // Education
    let mut education = String::new();
    education.push_str("\\newSection{Education}\n");
    for entry in &resume.education {
        education.push_str("\\newEntity{");
        education.push_str(&entry.institution);
        education.push_str("}{");
        education.push_str(&entry.location);
        education.push_str("}{");
        education.push_str(&entry.title);
        education.push_str("}{");
        education.push_str(&entry.end_date);
        education.push_str("}\n");
    }
    latex = latex.replace("{{education}}", &education);

    // Save string to resume.tex
    write("assets/resume.tex", latex.clone()).unwrap();
}

fn create_pdf() {
    // create the template map
    let mut dict = HashMap::new();
    dict.insert("test".into(), "Minimal".into());
    // provide the folder where the file for latex compiler are found
    let input = LatexInput::from("assets");
    // create a new clean compiler environment and the compiler wrapper
    let compiler = LatexCompiler::new(dict).unwrap();
    // run the underlying pdflatex or whatever
    let result = compiler.run("assets/resume.tex", &input).unwrap();

    // copy the file into the working directory
    let output = ::std::env::current_dir().unwrap().join("resume.pdf");
    assert!(write(output, result).is_ok());
}

pub fn generate(resume: &Resume) {
    println!("Generating resume...");

    // Create the LaTeX file
    create_latex(resume);

    // Create the PDF
    create_pdf();
}
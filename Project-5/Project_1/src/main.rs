// Rust program to validate Public Servant APS level

use std::io;

fn main() {
    // Data stored in vectors
    let aps_levels = vec![
    ("APS 1-2", vec![
        ("office Administrator", "Intern"),
        ("Academic", "_"),
        ("Lawyer", "Paralegal"),
        ("Teacher", "Placement"),
        ]),
    ("APS 3-5", vec![
        ("office Administrator", "Administrator"),
        ("Academic", "Research Assistant"),
        ("Lawyer", "Junior Associate"),
        ("Teacher", "Classroom Teacher"),
        ]),
    ("APS 5-8", vec![
        ("office Administrator", "Senior Administrator"),
        ("Academic", "PHD Candidate"),
        ("Lawyer", "Associate"),
        ("Teacher", "Senior Teacher"),
        ]),
    ("EL1 8-10", vec![
        ("office Administrator", "Office Manager"),
        ("Academic", "Post-Doc Researcher"),
        ("Lawyer", "Senior Associate 1-2"),
        ("Teacher", "Leading Teacher"),
        ]),
    ("EL2 10-13", vec![
        ("office Administrator", "Director"),
        ("Academic", "Senior Lecture"),
        ("Lawyer", "Senior Associate 3-4"),
        ("Teacher", "Deputy principal"),
        ]),
    ("SES", vec![
        ("office Administrator", "CEO"),
        ("Academic", "Dean"),
        ("Lawyer", "Partner"),
        ("Teacher", "Principal"),
        ]),
    ];

    // Print APS level table
    println!("APS level table");

    for (aps, rows) in &aps_levels {
        println!("\n{}:", aps);
        for (profession, title) in rows {
            println!(" {:22}, {}", profession, title);
        }
    }

    // User Input
    let mut profession = String::new();
    let mut job_title = String::new();
    let mut experience_input = String::new();

    println!("Enter Profession (e.g., Office Administrator, Lawyer,Teacher): ");
    io::stdin().read_line(&mut profession).unwrap();
    let profession = profession.trim();

    println!("Enter Job_title (e.g., Associate,Principal): ");
    io::stdin().read_line(&mut job_title).unwrap();
    let job_title = job_title.trim();


    println!("Enter Years of Experience: ");
    io::stdin().read_line(&mut experience_input).unwrap();
    let years: u32 = experience_input.trim().parse().unwrap_or(0);

    // Check APS level
    let mut result = "Staff level not found";

    for (aps, entries) in aps_levels {
        for (prof, title) in entries {
            if prof.eq_ignore_ascii_case(profession)
            && title.eq_ignore_ascii_case(job_title)
            {
                // Example rule: experience must fall inside APS level range
                // (Simple logic - you can expand this)
                if years >= 1 {
                    result = aps;
                }
            }
        }
    }

    // Output Result
    println!("\n====================");
    println!("Profession: {}", profession);
    println!("Job_title: {}", job_title);
    println!("Years of Experience: {} Years", years);
    println!("APS level: {}", result);



}

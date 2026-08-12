// Your enum here
enum JobStatus {
    Applied,
    Interviewing,
    Offered,
    Rejected,
}

// Your struct here
struct Candidate {
    name: String,
    status: JobStatus,
}

fn main() {
    let candidate = Candidate {
        name: String::from("Alice"),
        status: JobStatus::Interviewing,
    };

    // match block here
    match candidate.status {
        JobStatus::Applied => println!("{} has applied.", candidate.name),
        JobStatus::Interviewing => println!("{} is interviewing.", candidate.name),
        JobStatus::Offered => println!("{} has been offered the job.", candidate.name),
        JobStatus::Rejected => println!("{} has been rejected.", candidate.name),
    }
}

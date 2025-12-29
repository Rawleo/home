#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub tag: &'static str,
    pub description: &'static str,
    pub overview: &'static str,
    pub role: &'static str,
    pub technologies: Vec<&'static str>,
    pub live_link: Option<&'static str>,
    pub code_link: Option<&'static str>,
    pub paper_link: Option<&'static str>,
    pub posters: Option<Vec<Poster>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Poster {
    pub name: &'static str,
    pub url: &'static str,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            id: "genezippers",
            title: "GeneZippers: DNA Compression",
            subtitle: "From Bases to Bits: An Analysis of Early DNA Compression Algorithms",
            tag: "Research",
            description: "Analysis of early DNA compression algorithms comparing Huffman Coding, DNAzip, and Biocompress 1 for genomic data.",
            overview: "This project evaluates three distinct data compression strategies to assess their efficacy in handling massive genomic datasets by comparing a general text-based approach (Huffman Coding) against two specialized DNA compressors: DNAzip (reference-based) and Biocompress 1 (non-reference-based).",
            role: "I focused on implementing and analyzing DNAzip, a reference-based DNA compression algorithm. This involved understanding the algorithm's approach to leveraging sequence similarity and evaluating its performance across different genomic datasets.",
            technologies: vec!["Python", "Bioinformatics", "Data Compression", "Algorithm Analysis"],
            live_link: Some("https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/"),
            code_link: Some("https://github.com/Rawleo/genezippers_comps"),
            paper_link: Some("https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/paper.html"),
            posters: Some(vec![
                Poster { name: "Gavin: Biocompress 1", url: "https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/img/posters/Saxer_Poster.pdf" },
                Poster { name: "Jared: DNAzip", url: "https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/img/posters/ArroyoRuiz_Poster.pdf" },
                Poster { name: "Ryan: DNAzip", url: "https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/img/posters/Son_Poster.pdf" },
            ]),
        },
        Project {
            id: "portfolio",
            title: "Portfolio Website",
            subtitle: "Modern, High-Performance Web Development",
            tag: "Web App",
            description: "A high-performance portfolio website built with Rust and Leptos, featuring modern design and seamless navigation.",
            overview: "Built to demonstrate the capabilities of WebAssembly and Rust in frontend development. This site features fine-grained reactivity, signal-based state management, and a custom router implementation. It is fully typed and compiled to WASM for near-native performance.",
            role: "Full-Stack Developer",
            technologies: vec!["Rust", "Leptos", "WASM", "CSS3", "Trunk"],
            live_link: None,
            code_link: None,
            paper_link: None,
            posters: None,
        },
        Project {
            id: "cli-tool",
            title: "CLI Development Tool",
            subtitle: "Automating Developer Workflows",
            tag: "Tool",
            description: "Developer productivity tool that automates common workflows and integrates with popular CI/CD platforms.",
            overview: "A command-line interface tool designed to streamline the daily tasks of developers. It handles scaffolding, configuration management, and deployment pipelines, reducing boilerplate code and manual errors.",
            role: "Systems Engineer",
            technologies: vec!["Rust", "Clap", "Tokio", "GitHub Actions"],
            live_link: None,
            code_link: None,
            paper_link: None,
            posters: None,
        },
    ]
}

pub fn get_project_by_id(id: &str) -> Option<Project> {
    get_projects().into_iter().find(|p| p.id == id)
}
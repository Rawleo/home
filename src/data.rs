#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub tag: &'static str,
    pub description: &'static str,
    pub overview: &'static str,
    pub role: &'static str,
    pub technologies: Option<Vec<&'static str>>,
    pub live_link: Option<&'static str>,
    pub code_link: Option<&'static str>,
    pub paper_link: Option<&'static str>,
    pub posters: Option<Vec<Poster>>,
    pub photos: Option<Vec<Photo>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Poster {
    pub name: &'static str,
    pub url: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Photo {
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
            technologies: Some(vec!["Python", "Bioinformatics", "Data Compression", "Algorithm Analysis"]),
            live_link: Some("https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/"),
            code_link: Some("https://github.com/Rawleo/genezippers_comps"),
            paper_link: Some("https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/paper.html"),
            posters: Some(vec![
                Poster { name: "Gavin: Biocompress 1", url: "https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/img/posters/Saxer_Poster.pdf" },
                Poster { name: "Jared: DNAzip", url: "https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/img/posters/ArroyoRuiz_Poster.pdf" },
                Poster { name: "Ryan: DNAzip", url: "https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/img/posters/Son_Poster.pdf" },
            ]),
            photos: None,
        },
        Project {
            id: "portfolio",
            title: "Portfolio Website",
            subtitle: "Modern, High-Performance Web Development",
            tag: "Web App",
            description: "A high-performance portfolio website built with Rust and Leptos, featuring modern design and seamless navigation.",
            overview: "Built to demonstrate the capabilities of WebAssembly and Rust in frontend development. This site features fine-grained reactivity, signal-based state management, and type-safe routing. It is fully typed and compiled to WASM for near-native performance.",
            role: "Full-Stack Developer",
            technologies: Some(vec!["Rust", "Leptos", "WASM", "SCSS", "Trunk"]),
            live_link: Some("https://rawleo.github.io/home/"),
            code_link: Some("https://github.com/Rawleo/home"),
            paper_link: None,
            posters: None,
            photos: None,
        },
        Project {
            id: "spotwelder",
            title: "DIY Spotwelder",
            subtitle: "Microwave Transformer to Battery Spotwelder",
            tag: "DIY",
            description: "Revitalizing a microwave transformer into a battery making powerhouse.",
            overview: "Built to construct custom designed battery packs for my electrical vehicles.",
            role: "Builder",
            technologies: Some(vec!["Electricity", "Magnetism"]),
            live_link: None,
            code_link: None,
            paper_link: None,
            posters: None,
            photos: Some(vec![
                Photo { url: "images/SpotWelderBare.jpg"},
                Photo { url: "images/SpotWelderTop.jpg"},
                Photo { url: "images/SpotWelderFront.jpg"},
                Photo { url: "images/CoilRemoval.jpg"},
            ]),
        },
        // Project {
        //     id: "Test Project",
        //     title: "Test Title",
        //     subtitle: "Test Subtitle",
        //     tag: "Test Tag",
        //     description: "Test Description",
        //     overview: "Test Overview",
        //     role: "Test Role",
        //     technologies: vec!["Test Technology"],
        //     live_link: None,
        //     code_link: None,
        //     paper_link: None,
        //     posters: None,
        // },
    ]
}

pub fn get_project_by_id(id: &str) -> Option<Project> {
    get_projects().into_iter().find(|p| p.id == id)
}
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>

        // sets the document title
        <Title text="Portfolio | Rust Developer"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("project/genezippers") view=GeneZippersProject/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Navbar/>
        <Hero/>
        <Projects/>
        <Photos/>
        <Footer/>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    view! {
        <nav>
            <div class="logo">"Portfolio"</div>
            <ul class="nav-links">
                <li><a href="#home">"Home"</a></li>
                <li><a href="#projects">"Projects"</a></li>
                <li><a href="#photos">"Photos"</a></li>
                <li><a href="#contact">"Contact"</a></li>
            </ul>
        </nav>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero" id="home">
            <div class="container hero-content">
                <img src="/images/headshot.jpg" alt="Profile" class="hero-image"/>
                <h1>"Building the Future"</h1>
                <p>"Full-stack developer crafting high-performance web applications with Rust, modern frameworks, and cutting-edge technologies."</p>
                <a href="#projects" class="btn btn-primary">"View My Work"</a>
            </div>
        </section>
    }
}

#[component]
fn Projects() -> impl IntoView {
    let projects = vec![
        ProjectData {
            tag: "Research",
            title: "GeneZippers: DNA Compression",
            description: "Analysis of early DNA compression algorithms comparing Huffman Coding, DNAzip, and Biocompress 1 for genomic data.",
            link: Some("/project/genezippers"),
        },
        ProjectData {
            tag: "Web App",
            title: "Portfolio Website",
            description: "A high-performance portfolio website built with Rust and Leptos, featuring modern design and seamless navigation.",
            link: None,
        },
        ProjectData {
            tag: "Tool",
            title: "CLI Development Tool",
            description: "Developer productivity tool that automates common workflows and integrates with popular CI/CD platforms.",
            link: None,
        },
    ];

    view! {
        <section class="projects container" id="projects">
            <h2 class="section-title">"Featured Projects"</h2>
            <div class="projects-grid">
                {projects.into_iter().map(|project| {
                    view! { <ProjectCard project=project/> }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

#[component]
fn ProjectCard(project: ProjectData) -> impl IntoView {
    let card_content = view! {
        <span class="tag">{project.tag}</span>
        <h3>{project.title}</h3>
        <p>{project.description}</p>
    };

    if let Some(link) = project.link {
        view! {
            <a href={link} class="project-card">
                {card_content}
            </a>
        }.into_any()
    } else {
        view! {
            <div class="project-card">
                {card_content}
            </div>
        }.into_any()
    }
}

#[derive(Clone)]
struct ProjectData {
    tag: &'static str,
    title: &'static str,
    description: &'static str,
    link: Option<&'static str>,
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="container">
                <p>"© 2025 Portfolio. Built with Rust & Leptos."</p>
                <div class="footer-links">
                    <a href="https://github.com" target="_blank">"GitHub"</a>
                    <a href="https://linkedin.com" target="_blank">"LinkedIn"</a>
                    <a href="mailto:hello@example.com">"Email"</a>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn Photos() -> impl IntoView {
    view! {
        <section class="photos container" id="photos">
            <h2 class="section-title">"Photos"</h2>
            <div class="photos-grid">
                <div class="photo-card">
                    <img src="/images/temple-photo.jpg" alt="Temple at sunrise"/>
                    <div class="photo-caption">"Angkor Wat, Cambodia"</div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn GeneZippersProject() -> impl IntoView {
    view! {
        <div>
            <Navbar/>
            <section class="project-detail">
                <div class="container">
                    <a href="/" class="back-link">"← Back to Portfolio"</a>
                    
                    <div class="project-header">
                        <span class="tag">"Research"</span>
                        <h1>"GeneZippers: DNA Compression Analysis"</h1>
                        <p class="project-subtitle">"From Bases to Bits: An Analysis of Early DNA Compression Algorithms"</p>
                    </div>

                    <div class="project-content">
                        <div class="project-section">
                            <h2>"Overview"</h2>
                            <p>"This project evaluates three distinct data compression strategies to assess their efficacy in handling massive genomic datasets by comparing a general text-based approach (Huffman Coding) against two specialized DNA compressors: DNAzip (reference-based) and Biocompress 1 (non-reference-based)."</p>
                        </div>

                        <div class="project-section">
                            <h2>"My Role"</h2>
                            <p>"I focused on implementing and analyzing DNAzip, a reference-based DNA compression algorithm. This involved understanding the algorithm's approach to leveraging sequence similarity and evaluating its performance across different genomic datasets."</p>
                        </div>

                        <div class="project-section">
                            <h2>"Technologies"</h2>
                            <div class="tech-tags">
                                <span class="tech-tag">"Python"</span>
                                <span class="tech-tag">"Bioinformatics"</span>
                                <span class="tech-tag">"Data Compression"</span>
                                <span class="tech-tag">"Algorithm Analysis"</span>
                            </div>
                        </div>

                        <div class="project-section">
                            <h2>"Paper"</h2>
                            <div class="project-links">
                                <a href="https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/paper.html" target="_blank" class="btn btn-primary">"Read the Paper"</a>
                            </div>
                        </div>

                        <div class="project-section">
                            <h2>"Repositories"</h2>
                            <div class="project-links">
                                <a href="https://github.com/Rawleo/genezippers_comps" target="_blank" class="btn btn-primary">"GeneZippers Codebase"</a>
                                <a href="https://github.com/Rawleo/genezippers_web" target="_blank" class="btn btn-secondary">"GeneZippers Web"</a>
                            </div>
                        </div>

                        <div class="project-section">
                            <h2>"Posters"</h2>
                            <div class="posters-grid">
                                <a href="https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/img/posters/Saxer_Poster.pdf" target="_blank" class="btn btn-secondary">"Gavin: Biocompress 1"</a>
                                <a href="https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/img/posters/ArroyoRuiz_Poster.pdf" target="_blank" class="btn btn-secondary">"Jared: DNAzip"</a>
                                <a href="https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/img/posters/Son_Poster.pdf" target="_blank" class="btn btn-secondary">"Ryan: DNAzip"</a>
                            </div>
                        </div>

                        <div class="project-section">
                            <h2>"Links"</h2>
                            <div class="project-links">
                                <a href="https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/" target="_blank" class="btn btn-primary">"View Project Website"</a>
                                <a href="https://github.com/Rawleo/genezippers_comps" target="_blank" class="btn btn-secondary">"View Code"</a>
                            </div>
                        </div>
                        <p class="original-link">"Original site: "<a href="https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/" target="_blank">"https://www.cs.carleton.edu/cs_comps/2526/genezippers_web/website/"</a></p>
                    </div>
                </div>
            </section>
            <Footer/>
        </div>
    }
}


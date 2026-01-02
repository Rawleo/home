use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    hooks::{use_location, use_params_map},
    path, // <--- Crucial import for path! macro
};

use crate::data::{get_project_by_id, get_projects, Project};

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
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Title text="Ryan Son | Rust Developer"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/project/:id") view=ProjectLoader/>
                    <Route path=path!("/about") view=AboutPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn ProjectLoader() -> impl IntoView {
    let params = use_params_map();

    let project_data = move || {
        let id = params.get().get("id").unwrap_or_default();
        get_project_by_id(&id)
    };

    view! {
        {move || match project_data() {
            Some(data) => view! { <ProjectDetail project=data/> }.into_any(),
            None => view! {
                <div>
                    <Navbar/>
                    <div class="container" style="padding-top: 100px; text-align: center;">
                        <h1>"Project Not Found"</h1>
                        <p>"The project you are looking for does not exist."</p>
                        <a href="/" class="btn btn-primary">"Return Home"</a>
                    </div>
                    <Footer/>
                </div>
            }.into_any(),
        }}
    }
}

#[component]
fn ProjectDetail(project: Project) -> impl IntoView {
    view! {
        <div>
            <Navbar/>
            <section class="project-detail">
                <div class="container">
                    <a href="/#projects" class="back-link">"← Back to Portfolio"</a>

                    <div class="project-header">
                        <span class="tag">{project.tag}</span>
                        <h1>{project.title}</h1>
                        <p class="project-subtitle">{project.subtitle}</p>
                    </div>

                    <div class="project-content">
                        <div class="project-section">
                            <h2>"Overview"</h2>
                            <p>{project.overview}</p>
                        </div>

                        <div class="project-section">
                            <h2>"My Role"</h2>
                            <p>{project.role}</p>
                        </div>

                        <div class="project-section">
                            <h2>"Technologies"</h2>
                            <div class="tech-tags">
                                <For
                                    each=move || project.technologies.clone()
                                    key=|tech| tech.to_string()
                                    children=|tech| view! { <span class="tech-tag">{tech}</span> }
                                />
                            </div>
                        </div>

                        {move || project.posters.clone().map(|posters| view! {
                             <div class="project-section">
                                <h2>"Posters"</h2>
                                <div class="posters-grid">
                                    <For
                                        each=move || posters.clone()
                                        key=|poster| poster.name
                                        children=|poster| view! {
                                            <a href=poster.url target="_blank" class="btn btn-secondary">{poster.name}</a>
                                        }
                                    />
                                </div>
                            </div>
                        })}

                        <div class="project-section">
                            <h2>"Resources"</h2>
                            <div class="project-links">
                                {move || project.paper_link.map(|link| view! {
                                    <a href=link target="_blank" class="btn btn-primary">"Read Paper"</a>
                                })}
                                {move || project.code_link.map(|link| view! {
                                    <a href=link target="_blank" class="btn btn-secondary">"View Code"</a>
                                })}
                                {move || project.live_link.map(|link| view! {
                                    <a href=link target="_blank" class="btn btn-secondary">"Live Site"</a>
                                })}
                                {move || project.live_link.map(|link| view! {
                                    <a href=link target="_blank" class="btn btn-secondary">"Live Site"</a>
                                })}
                            </div>
                        </div>
                    </div>
                </div>
            </section>
            <Footer/>
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let location = use_location();

    Effect::new(move |_| {
        let hash = location.hash.get();

        if !hash.is_empty() {
            let hash_string = hash.to_string();

            request_animation_frame(move || {
                let id = hash_string.trim_start_matches('#');
                if let Some(el) = document().get_element_by_id(id) {
                    el.scroll_into_view();
                }
            });
        }
    });

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
    let location = use_location();
    let (current_hash, set_current_hash) = signal(String::new());

    Effect::new(move |_| {
        set_current_hash.set(location.hash.get());
    });

    let scroll_to = move |target_id: &str| {
        if location.pathname.get() == "/" {
            let target_id_owned = target_id.to_string();

            request_animation_frame(move || {
                if let Some(el) = document().get_element_by_id(&target_id_owned) {
                    el.scroll_into_view();
                }
            });
        }
    };

    let is_active = move |path: &str| {
        let current_path = location.pathname.get();
        if path == "/" { current_path == "/" } else { current_path.starts_with(path) }
    };

    view! {
        <nav>
            <div class="logo">
                <a href="/">"Ryan Son"</a>
            </div>
            <ul class="nav-links">
                <li>
                    <a href="/#home"
                       class:active=move || is_active("/")
                       on:click=move |_| scroll_to("home")>"Home"</a>
                </li>
                <li>
                    <a href="/#projects"
                       class:active=move || is_active("/project") || current_hash.get() == "#projects"
                       on:click=move |_| scroll_to("projects")>"Projects"</a>
                </li>
                <li>
                    <a href="/#photos"
                       class:active=move || is_active("/photos") || current_hash.get() == "#photos"
                       on:click=move |_| scroll_to("photos")>"Photos"</a>
                </li>
                // <li>
                //     <a href="/#contact"
                //        class:active=move || is_active("/contact") || current_hash.get() == "#contact"
                //        on:click=move |_| scroll_to("contact")>"Contact"</a>
                // </li>
                <li>
                    <a href="/about" class:active=move || is_active("/about")>"About"</a>
                </li>
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
                <h1>"Scalable Web Apps"</h1>
                <p>"Full-stack developer crafting high-performance web applications with Rust and other modern frameworks."</p>
                <div class="hero-links">
                    <a href="/#projects" class="btn btn-primary hero-project-btn">"View My Work"</a>
                    <a href="https://github.com/rawleo" class="btn btn-secondary" target="_blank">"GitHub"</a>
                    <a href="https://www.linkedin.com/in/ryanson50" class="btn btn-secondary" target="_blank">"LinkedIn"</a>
                    <a href="mailto:sonryan50@gmail.com" class="btn btn-secondary">"Email"</a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Projects() -> impl IntoView {
    let projects = get_projects();

    view! {
        <section class="projects container" id="projects">
            <h2 class="section-title">"Projects"</h2>
            <div class="projects-grid">
                {projects.into_iter().map(|project| {
                    view! { <ProjectCard project=project/> }
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    let link = format!("/project/{}", project.id);

    view! {
        <a href=link class="project-card">
            <span class="tag">{project.tag}</span>
            <h3>{project.title}</h3>
            <p>{project.description}</p>
        </a>
    }
}

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <Navbar/>
            <About/>
            <Footer/>
        </div>
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
fn About() -> impl IntoView {
    view! {
        <section class="about section-container" id="about">
            <div class="container">
                <h2 class="section-title">"About Me"</h2>
                <div class="about-grid">
                    <div class="about-image-wrapper">
                        <img src="/images/headshot.jpg" alt="Ryan Son" class="about-image"/>
                    </div>
                    <div class="about-text">
                        <p>"Hello! I'm Ryan Son, a junior full-stack developer and soon to be graduate of Carleton College. I'm passionate about crafting scalable web applications with Rust and other modern frameworks."</p>
                        <p>"My journey in software development is driven by a curiosity for how complex systems work and a desire to build tools that make a real impact. With a background in research and a keen interest in high-performance computing, I find Rust to be the perfect tool for building robust, efficient software."</p>
                        <p>"When I'm not coding, you can find me exploring new technologies, home-labbing, or capturing the world through my lens (as seen in my Photos section!)."</p>
                        <p>"I'm currently looking for full-time opportunities, so feel free to reach out!"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

// #[component]
// fn Contact() -> impl IntoView {
//     view! {
//         <section class="contact-container" id="social-links">
//             // <h2 class="section-title">"Contact Links"</h2>
//
//         </section>
//     }
// }

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="container">
                <p>"© 2025 Portfolio. Built with Rust & Leptos."</p>
                <div class="footer-links">
                    <a href="https://github.com/rawleo" target="_blank">"GitHub"</a>
                    <a href="https://www.linkedin.com/in/ryanson50" target="_blank">"LinkedIn"</a>
                    <a href="mailto:sonryan50@gmail.com">"Email"</a>
                </div>
            </div>
        </footer>
    }
}
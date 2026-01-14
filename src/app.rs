use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{A, Route, Router, Routes},
    hooks::{use_location, use_params_map},
    path,
};

use crate::data::{get_project_by_id, get_projects, Project, get_blog_by_id, get_blogs, Blog, get_photos, Photo};

#[derive(Copy, Clone, Debug)]
struct LightboxState(WriteSignal<Option<String>>);

#[derive(Clone, Debug)]
pub struct BasePath(pub String);

impl BasePath {
    pub fn path(&self, path: &str) -> String {
        let path = path.trim_start_matches('/');
        if self.0.is_empty() {
            format!("/{}", path)
        } else {
            format!("{}/{}", self.0, path)
        }
    }
}

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
    let (selected_image, set_selected_image) = signal(None::<String>);
    provide_context(LightboxState(set_selected_image));

    // Get base path from the HTML <base> tag or fallback to location
    let base_path = document()
        .query_selector("base")
        .ok()
        .flatten()
        .and_then(|base| base.get_attribute("href"))
        .unwrap_or_else(|| {
            let path = window().location().pathname().unwrap_or_default();
            if path.starts_with("/home") {
                "/home/".to_string()
            } else {
                "/".to_string()
            }
        });

    // Leptos Router base should be the prefix without the trailing slash
    let router_base = if base_path == "/" {
        "".to_string()
    } else {
        base_path.trim_end_matches('/').to_string()
    };
    provide_context(BasePath(router_base.clone()));

    view! {
        <Stylesheet id="leptos" href="pkg/portfolio.css"/>
        <Title text="Ryan Son | Full-Stack Developer"/>

        <Router base=router_base>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/project/:id") view=ProjectLoader/>
                    <Route path=path!("/blog/:id") view=BlogLoader/>
                    <Route path=path!("/projects") view=ProjectsPage/>
                    <Route path=path!("/blog") view=BlogPage/>
                    <Route path=path!("/photos") view=PhotosPage/>
                    <Route path=path!("/about") view=AboutPage/>
                </Routes>
            </main>
            <Lightbox selected_image=selected_image set_selected_image=set_selected_image/>
        </Router>
    }
}

#[component]
fn Lightbox(
    selected_image: ReadSignal<Option<String>>,
    set_selected_image: WriteSignal<Option<String>>
) -> impl IntoView {
    view! {
        {move || selected_image.get().map(|url| view! {
            <div class="lightbox-overlay" on:click=move |_| set_selected_image.set(None)>
                <div class="lightbox-content" on:click=move |ev| ev.stop_propagation()>
                    <img src=url alt="Full size photo"/>
                    <button class="lightbox-close" on:click=move |_| set_selected_image.set(None)>"×"</button>
                </div>
            </div>
        })}
    }
}

#[component]
pub fn ProjectLoader() -> impl IntoView {
    let params = use_params_map();

    let project_data = move || {
        let id = params.get().get("id").unwrap_or_default();
        get_project_by_id(&id)
    };

    let base = use_context::<BasePath>().expect("BasePath context not found");

    view! {
        {move || match project_data() {
            Some(data) => view! { <ProjectDetail project=data/> }.into_any(),
            None => view! {
                <div>
                    <Navbar/>
                    <div class="container" style="padding-top: 100px; margin-bottom: 30px; text-align: center;">
                        <h1>"Project Not Found"</h1>
                        <p>"The project you are looking for does not exist."</p>
                        <A href=base.path("/") attr:style="margin-top: 20px" attr:class="btn btn-primary">"Return Home"</A>
                    </div>
                    <Footer/>
                </div>
            }.into_any(),
        }}
    }
}

#[component]
fn ProjectDetail(project: Project) -> impl IntoView {
    let base = use_context::<BasePath>().expect("BasePath context not found");
    view! {
        <div>
            <Navbar/>
            <section class="project-detail">
                <div class="container">
                    <A href=base.path("/#projects") attr:class="back-link">"← Back to Portfolio"</A>

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
                                    each=move || project.technologies.clone().unwrap_or_else(Vec::new)
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
                            {
                                let photo_urls = project.photos.clone().map(|photos| {
                                    photos
                                        .iter()
                                        .map(|p| p.url.to_string())
                                        .collect::<Vec<String>>()
                                });

                                view! {
                                    <div class="project-section">
                                        {if let Some(urls) = photo_urls {
                                            view! { <Slideshow images=urls/> }
                                                .into_any()
                                        } else {
                                            view! {}
                                                .into_any()
                                        }}
                                    </div>
                                }
                            }

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
        <Blogs/>
        <Photos/>
        <Footer/>
    }
}

#[component]
fn Navbar() -> impl IntoView {
    let location = use_location();
    let (current_hash, set_current_hash) = signal(String::new());
    let (is_open, set_is_open) = signal(false);

    Effect::new(move |_| {
        set_current_hash.set(location.hash.get());
    });

    let scroll_to = move |target_id: &str| {
        set_is_open.set(false);
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

    let base = use_context::<BasePath>().expect("BasePath context not found");

    view! {
        <nav>
            <div class="logo">
                <A href=base.path("/") on:click=move |_| set_is_open.set(false)>"Ryan Son"</A>
            </div>

            <button
                class="menu-toggle"
                class:open=move || is_open.get()
                on:click=move |_| set_is_open.update(|n| *n = !*n)
                aria-label="Toggle menu"
            >
                <span></span>
                <span></span>
                <span></span>
            </button>

            <ul class="nav-links" class:open=move || is_open.get()>
                <li>
                    <A href=base.path("/")
                       class:active=move || is_active("/")
                       on:click=move |_| scroll_to("home")>"Home"</A>
                </li>
                <li>
                    <A href=base.path("/projects")
                       class:active=move || is_active("/projects") || is_active("/project") || current_hash.get() == "#projects"
                       on:click=move |_| scroll_to("projects")>"Projects"</A>
                </li>
                <li>
                    <A href=base.path("/blog")
                       class:active=move || is_active("/blog") || current_hash.get() == "#blogs"
                       on:click=move |_| scroll_to("blogs")>"Blogs"</A>
                </li>
                <li>
                    <A href=base.path("/photos")
                       class:active=move || is_active("/photos") || current_hash.get() == "#photos"
                       on:click=move |_| scroll_to("photos")>"Photos"</A>
                </li>
                <li>
                    <A href=base.path("/about")
                       class:active=move || is_active("/about")
                       on:click=move |_| set_is_open.set(false)>"About"</A>
                </li>
            </ul>
        </nav>
    }
}

#[component]
fn Hero() -> impl IntoView {
    let base = use_context::<BasePath>().expect("BasePath context not found");
    view! {
        <section class="hero" id="home">
            <div class="container hero-content">
                <img src="images/headshot.jpg" alt="Profile" class="hero-image"/>
                <h1>"From frontend to backend—designed to scale"</h1>
                <p>"Full-stack developer crafting high-performance applications."</p>
                <div class="hero-links">
                    <A href=base.path("/#projects") attr:class="btn btn-primary hero-project-btn">"View My Work"</A>
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
    let base = use_context::<BasePath>().expect("BasePath context not found");

    view! {
        <section class="projects container" id="projects">
            <h2 class="section-title">"Featured Projects"</h2>
            <div class="projects-grid">
                {projects.into_iter().take(3).map(|project| {
                    view! {
                        <Card
                            id=project.id
                            title=project.title
                            description=project.description
                            tag=project.tag
                            base_path="project"
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div style="text-align: center; margin-top: 3rem;">
                <A href=base.path("/projects") attr:class="btn btn-secondary">"View All Projects"</A>
            </div>
        </section>
    }
}

#[component]
fn Card(
    id: &'static str,
    title: &'static str,
    description: &'static str,
    tag: &'static str,
    base_path: &'static str,
) -> impl IntoView {
    let base = use_context::<BasePath>().expect("BasePath context not found");
    let link = base.path(&format!("{}/{}", base_path, id));

    view! {
        <A href=link attr:class="project-card">
            <span class="tag">{tag}</span>
            <h3>{title}</h3>
            <p>{description}</p>
        </A>
    }
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects = get_projects();

    view! {
        <div>
            <Navbar/>
            <section class="projects container" style="padding-top: 120px;">
                <h1 class="section-title">"All Projects"</h1>
                <div class="projects-grid">
                    {projects.into_iter().map(|project| {
                        view! {
                            <Card
                                id=project.id
                                title=project.title
                                description=project.description
                                tag=project.tag
                                base_path="project"
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </section>
            <Footer/>
        </div>
    }
}

#[component]
pub fn BlogPage() -> impl IntoView {
    let blogs = get_blogs();

    view! {
        <div>
            <Navbar/>
            <section class="projects container" style="padding-top: 120px;">
                <h1 class="section-title">"Blog"</h1>
                <div class="projects-grid">
                    {blogs.into_iter().map(|blog| {
                        view! {
                            <Card
                                id=blog.id
                                title=blog.title
                                description=blog.description
                                tag=blog.tag
                                base_path="blog"
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </section>
            <Footer/>
        </div>
    }
}

#[component]
fn PhotoCard(photo: Photo) -> impl IntoView {
    let setter = use_context::<LightboxState>().expect("LightboxState context not found").0;
    let url = photo.url.to_string();

    view! {
        <div class="photo-card" on:click=move |_| setter.set(Some(url.clone()))>
            <img src=photo.url alt=photo.caption/>
            <div class="photo-caption">{photo.caption}</div>
        </div>
    }
}

#[component]
pub fn PhotosPage() -> impl IntoView {
    let photos = get_photos();

    view! {
        <div>
            <Navbar/>
            <section class="photos container" style="padding-top: 120px;">
                <h1 class="section-title">"Photos"</h1>
                <div class="photos-grid">
                    {photos.into_iter().map(|photo| {
                        view! {
                            <PhotoCard photo=photo />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </section>
            <Footer/>
        </div>
    }
}

#[component]
fn Blogs() -> impl IntoView {
    let blogs = get_blogs();
    let base = use_context::<BasePath>().expect("BasePath context not found");

    view! {
        <section class="projects container" id="blogs">
            <h2 class="section-title">"Featured Blogs"</h2>
            <div class="projects-grid">
                {blogs.into_iter().take(3).map(|blog| {
                    view! {
                        <Card
                            id=blog.id
                            title=blog.title
                            description=blog.description
                            tag=blog.tag
                            base_path="blog"
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div style="text-align: center; margin-top: 3rem;">
                <A href=base.path("/blog") attr:class="btn btn-secondary">"View All Posts"</A>
            </div>
        </section>
    }
}



#[component]
pub fn BlogLoader() -> impl IntoView {
    let params = use_params_map();

    let blog_data = move || {
        let id = params.get().get("id").unwrap_or_default();
        get_blog_by_id(&id)
    };

    let base = use_context::<BasePath>().expect("BasePath context not found");

    view! {
        {move || match blog_data() {
            Some(data) => view! { <BlogDetail blog=data/> }.into_any(),
            None => view! {
                <div>
                    <Navbar/>
                    <div class="container" style="padding-top: 100px; margin-bottom: 30px; text-align: center;">
                        <h1>"Project Not Found"</h1>
                        <p>"The project you are looking for does not exist."</p>
                        <A href=base.path("/") attr:style="margin-top: 20px" attr:class="btn btn-primary">"Return Home"</A>
                    </div>
                    <Footer/>
                </div>
            }.into_any(),
        }}
    }
}

#[component]
fn BlogDetail(blog: Blog) -> impl IntoView {
    let base = use_context::<BasePath>().expect("BasePath context not found");
    view! {
        <div>
            <Navbar/>
            <section class="project-detail">
                <div class="container">
                    <A href=base.path("/#projects") attr:class="back-link">"← Back to Portfolio"</A>

                    <div class="project-header">
                        <span class="tag">{blog.tag}</span>
                        <h1>{blog.title}</h1>
                        <p class="project-subtitle">{blog.subtitle}</p>
                    </div>

                    <div class="project-content">
                        <div class="project-section">
                            <h2>"Overview"</h2>
                            <p>{blog.overview}</p>
                        </div>
                    </div>
                </div>
            </section>
            <Footer/>
        </div>
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
    let photos = get_photos();
    let base = use_context::<BasePath>().expect("BasePath context not found");

    view! {
        <section class="photos container" id="photos">
            <h2 class="section-title">"Photos"</h2>
            <div class="photos-grid">
                {photos.into_iter().take(3).map(|photo| {
                    view! { <PhotoCard photo=photo/> }
                }).collect::<Vec<_>>()}
            </div>
            <div style="text-align: center; margin-top: 3rem;">
                <A href=base.path("/photos") attr:class="btn btn-secondary">"View All Photos"</A>
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
                        <img src="images/headshot.jpg" alt="Ryan Son" class="about-image"/>
                    </div>
                    <div class="about-text">
                        <p>"Hello! I'm Ryan Son, a junior full-stack developer and soon to be graduate of Carleton College. I'm crazy passionate about crafting scalable, fast, and efficient applications."</p>
                        <p>"My journey in software development is driven by a curiosity for how complex systems work and a desire to build tools that make a real impact. With a background in research and a keen interest in high-performance computing, I love finding ways to improve the performance of my applications."</p>
                        <p>"When I'm not coding, you can find me exploring new technologies, home-labbing, or capturing the world through my lens (as seen in my Photos section!)."</p>
                        <p>"Thank you for stopping by!"</p>
                        <p>"I'm currently looking for full-time opportunities, so feel free to reach out!"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

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

#[component]
pub fn Slideshow(images: Vec<String>) -> impl IntoView {
    if images.is_empty() {
        return view! {}.into_any();
    }

    let len = images.len();
    let (index, set_index) = signal(0);
    let prev = move |_| set_index.update(|i| *i = if *i == 0 { len - 1 } else { *i - 1 });
    let next = move |_| set_index.update(|i| *i = (*i + 1) % len);
    let src = move || images.get(index.get()).cloned().unwrap_or_default();

    view! {
        <div class="slideshow">
            <button class="slide-btn slide-btn.prev" on:click=prev>"‹"</button>
            <img class="project-image" src=src />
            <button class="slide-btn slide-btn.next" on:click=next>"›"</button>
        </div>
    }.into_any()
}
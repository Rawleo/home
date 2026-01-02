# Portfolio

A personal portfolio website built with Rust and the Leptos web framework. This application showcases projects, photography, and professional background in a high-performance, single-page application.

## Features

- **Responsive Design**: Adapts seamlessly to mobile, tablet, and desktop screens.
- **Single Page Application (SPA)**: Smooth client-side routing and transitions.
- **Project Showcase**: Detailed views for individual projects with descriptions, tags, and links.
- **Photo Gallery**: A dedicated section for photography.
- **Dark Mode Aesthetic**: A clean, modern dark theme designed with SCSS.

## Technology Stack

- **Language**: Rust
- **Framework**: Leptos (v0.8.0)
- **Styling**: SCSS (Sass)
- **Build Tool**: Trunk
- **Router**: Leptos Router

## Getting Started

### Prerequisites

Ensure you have the following installed on your system:

- Rust toolchain (latest stable)
- Trunk (WASM bundler for Rust)
- Add the WASM target: `rustup target add wasm32-unknown-unknown`

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rusthome
   ```

2. Install dependencies (if any specific ones are needed beyond Cargo).

### Running the Application

To start the development server with Hot Module Replacement (HMR):

```bash
trunk serve --features csr
```

This will build the application and serve it at `http://localhost:8080`.

### Building for Production

To build optimized artifacts for deployment:

```bash
trunk build --features csr --release
```

The output files will be generated in the `dist` directory.

## Project Structure

- `src/`: Contains the Rust source code.
  - `app.rs`: Main application component, routing, and layout.
  - `data.rs`: static data definitions for projects and content.
  - `main.rs`: Entry point.
- `style/`: Contains SCSS stylesheets.
  - `main.scss`: Global styles and component styling.
- `public/`: Static assets like images and icons.

## License

This project is licensed under the terms found in the LICENSE file.

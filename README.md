# LetIt Rust SDK

A professional Rust client for the LetIt API, featuring high-performance support for **Microposts** and **Job** management.

## 📖 API Documentation

For detailed information on the underlying REST API, endpoints, and authentication schemas, please visit the official documentation:

- **API Reference**: [http://api.letit.com](https://api.letit.com/docs/client/)

## Features

- **Job Management**: Full support for creating job postings with company logos, descriptions, and metadata.
- **Micropost System**: Create text posts or file-based updates with attachment support.
- **Micropost Voting**: Toggle a vote on microposts using a single endpoint call.
- **Admin Blog Listing**: Retrieve public admin blog entries with filter and pagination query support.
- **Admin Blog Detail**: Fetch one public admin article by slug (nullable response).
- **Async HTTP Support**: Built on `reqwest` with centralized authentication and API error handling.

## Installation

```bash
cargo add letit
```

## Quick Start

### Initialize the Client

The client can be initialized with an explicit API key and base URL.

```rust
use letit::LetItClient;

fn main() {
    let client = LetItClient::new("https://api.letit.com", "your-api-token");
}
```

### Create a Job with a Company Logo

The SDK handles multipart form construction and file uploads automatically.

```rust
use letit::{CreateJobWithCompanyRequest, FilePayload, JobLocation, LetItClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LetItClient::new("https://api.letit.com", "your-api-token");

    let logo_bytes = std::fs::read("logo.png")?;
    let mut request = CreateJobWithCompanyRequest::new(
        "Acme Corp",
        "Building next-gen developer tools.",
        "https://acme.example",
        "Senior Rust Developer",
        "Building production SDKs and integrations.",
        "https://acme.example/careers",
    );
    request.company_logo = Some(FilePayload {
        filename: "logo.png".to_string(),
        bytes: logo_bytes,
        mime_type: Some("image/png".to_string()),
    });
    request.job_location = JobLocation::Remote;

    let response = client.job.create_with_company(request).await?;
    println!("Job created successfully: {}", response.slug);

    Ok(())
}
```

### Create a Micropost

Easily create posts with optional titles and bodies.

```rust
use letit::{CreateMicropostRequest, LetItClient, PostType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LetItClient::new("https://api.letit.com", "your-api-token");

    let mut request = CreateMicropostRequest::new("The Rust SDK is now live!");
    request.title = Some("New Update".to_string());
    request.post_type = PostType::Text;

    let response = client.micropost.create(request).await?;
    println!("Post created with ID: {}", response.public_id);

    Ok(())
}
```

### Vote a Micropost

Toggle the authenticated user's vote for a micropost.

```rust
use letit::LetItClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LetItClient::new("https://api.letit.com", "your-api-token");

    let result = client.micropost.vote("your-public-id").await?;
    println!("User voted: {}", result.user_voted);

    Ok(())
}
```

### List Admin Blogs

Retrieve admin blog entries with optional filters and pagination.

```rust
use letit::{LetItClient, ListAdminBlogsParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LetItClient::new("https://api.letit.com", "your-api-token");

    let params = ListAdminBlogsParams {
        title: Some("release".to_string()),
        category: None,
        skip: Some(0),
        limit: Some(10),
    };

    let response = client.blog.list_admin(params).await?;
    println!("Blogs: {}", response.list.len());

    Ok(())
}
```

### Get Admin Blog by Slug

Fetch one admin blog article by slug. The API can return `null` when not found.

```rust
use letit::LetItClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LetItClient::new("https://api.letit.com", "your-api-token");

    let blog = client.blog.get_admin("some-slug").await?;
    match blog {
        Some(article) => println!("Blog found: {}", article),
        None => println!("Blog not found"),
    }

    Ok(())
}
```

## Environment Variables

The SDK can utilize the following environment variables for testing or default configuration:

- `LETIT_API_TOKEN`: Required for authenticated integration tests against the live API.

## Testing

All integration tests target the production API at `https://api.letit.com`.

Because some tests create and then delete real resources, they are marked as ignored and require a valid `LETIT_API_TOKEN`.

Run the default compile-and-smoke pass:

```powershell
# In PowerShell
cargo test
```

```bash
# In Bash
cargo test
```

Run the live API integration suite explicitly:

```powershell
cargo test -- --ignored
```
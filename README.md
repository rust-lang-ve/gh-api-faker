<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/rust-lang-ve/design/main/assets/logo_above.png" height="120" width="120" />
  </div>
  <h1 align="center">gh-api-faker</h1>
  <h4 align="center">:octocat: API Faker for GitHub to avoid hitting the API request limit</h4>
</div>

## Running Locally

> This project only requires `cargo` to be installed in your system

```bash
cargo run
```

## Endpoints

Method | URL |
--- | ---
**GET** | `http://0.0.0.0:7878/orgs/{organization}/repos`
**GET** | `http://0.0.0.0:7878/users/{organization}/events`

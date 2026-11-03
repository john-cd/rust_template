# Website

A self-contained [Zola](https://www.getzola.org/) starter with no downloaded theme, JavaScript build
step, or external asset dependencies.

```sh
cargo make website-check
cargo make website-build
cargo tools website --serve
```

Edit Markdown under `content/`, Tera HTML under `templates/`, and assets under `static/`.
The generated `public/` directory is ignored. Configuration lives in `config.toml`.
Set `base_url` to your deployed URL before publication, or pass
`zola --root website build --base-url https://your-domain.example/` during deployment.
For a project hosted under a URL prefix, include that prefix in the base URL. Templates use
`get_url` and content permalinks so links and assets honor it.

CI checks and builds the site; deployment is intentionally left for the adopting project to configure.
See the [Zola CLI reference](https://www.getzola.org/documentation/getting-started/cli-usage/).

# model

Shared types for the ARES pipeline and bot: `Team`, `Event`, `Match`, `Alliance`, `current_season`, and the Supabase table names. One definition, so both sides agree on what a row means.

```toml
model = { git = "https://github.com/ares-hq/model.git", branch = "main" }
model = { git = "...", features = ["supabase"] }   # + PostgREST client
```

Everything is re-exported from `model::prelude`.

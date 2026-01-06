src/
  apk/
    mod.rs           ← exports only
    extract.rs
    fetch.rs
  catalog/
    mod.rs           ← exports only
    fetch.rs
    parse.rs
    resources.rs
    region/
      mod.rs
      japan.rs
      global.rs
  strategy/
    mod.rs
    table.rs
    media.rs
    asset.rs
  download/
    mod.rs           ← exports only
    downloader.rs
    filter.rs
  helpers/
    mod.rs           ← exports only
    types.rs         ← all the types (previously api.rs)
    config.rs
    error.rs
  lib.rs
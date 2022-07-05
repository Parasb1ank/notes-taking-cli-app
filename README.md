# notes-taking-cli-app
Rust CLI notes taking app with sqlite3

toml```
[dependencies.rusqlite]
version = "0.27"
features = ["bundled"]
```
This adds rusqlite as a dependency that will be installed the next time you attempt to build. features = [“bundled”] tells the package to compile SQLite . 

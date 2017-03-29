## Project Steps

Not: Sample configured on `Visual Studio Code`

* Create Project
Note: Don't use `-` charachter. If you use you can't debug your code.
```ssh
cargo new rust_redis_sample --bin
cd rust_redis_sample
```

* Build
```ssh
cargo build
```

#### Debugging (lldb)
* add .vscode/launch.json for debugging.
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug",
            "type": "lldb-mi",
            "request": "launch",
            "target": "target/debug/rust_redist_sample",
            "cwd": "${workspaceRoot}",
            "lldbmipath": "/Applications/Xcode.app/Contents/Developer/usr/bin/lldb-mi"
        }
    ]
}
```

* add .vscode/tasks.json for build code.


```json
{
    // See https://go.microsoft.com/fwlink/?LinkId=733558
    // for the documentation about the tasks.json format
    "version": "0.1.0",
    "isShellCommand": true,
    "args": [],
    "showOutput": "always",
    "tasks": [
        {
             //CHILDREN WITH COMMAND ;)
            "taskName": "cargo_build",
            "suppressTaskName": true,
            "isBuildCommand": true,
            "command": "cargo",
            "args": ["build"]
        }
    ]
}
```

* attach the added build task to debugging as preTask to build the code before debugging. 
// You need to add a "preLaunchTask": "cargo_build" to launch.json

```json 
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug",
            "type": "lldb-mi",
            "request": "launch",
            "target": "target/debug/rust_redist_sample",
            "cwd": "${workspaceRoot}",
            "lldbmipath": "/Applications/Xcode.app/Contents/Developer/usr/bin/lldb-mi",
            "preLaunchTask": "cargo_build"
        }
    ]
}
```

#### add redis-rs library to the dependency.

```yaml
[dependencies]
redis = "0.5.3"
```

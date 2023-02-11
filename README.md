# Wake ⏰

A simple (and WIP) command-line tool for waking up your complex projects and workflows.

Using `.wake` files, you specify a list of directories and commands to start as child processes.

Here is an example `.wake` file:

```
./api -> dotnet run
./front-end -> npm run
./logger -> docker compose up
```

The stdout and stderr of any child processes are asynchronously color-coded and streamed to your primary process.

The base functionality is already implemented, but future goals include:
 - Aliases for commands
 - Clearer output tagging in the primary process output
 - Optional command output redirection to files

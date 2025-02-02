# JellyRoller - The CLI Jellyfin Controller Utility for Linux and Windows

JellyRoller is an open source CLI Jellyfin Controller written in Rust that works on Windows and Linux. Its primary purpose is to allow administration of a Jellyfin application from the command line.

## How it works

Currently, JellyRoller authenticates as a user (admin rights required) and stores the authentication token within its configuration, similar to how an API key would be stored. Due to some issues with API keys in Jellyfin this is the only supported method of authentication. Once authenticated, JellyRoller uses the Jellyfin API to manage the server.

## Usage Information

```
jellyroller
A CLI controller for managing Jellyfin

USAGE:
    jellyroller <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    add-user                     Creates a new user
    add-users                    Uses the supplied file to mass create new users
    delete-user                  Deletes an existing user
    disable-user                 Disable a user
    enable-user                  Enable a user
    execute-task-by-name         Executes a scheduled task by name
    get-devices                  Show all active devices
    get-libraries                Gets the libraries available to the configured user
    get-plugins                  Returns a list of installed plugins
    get-scheduled-tasks          Show all scheduled tasks and their status
    grant-admin                  Grants the specified user admin rights
    help                         Print this message or the help of the given subcommand(s)
    list-logs                    Displays the available system logs
    list-users                   Lists the current users with basic information
    reconfigure                  Reconfigure the connection information
    remove-device-by-username    Removes all devices associated with the specified user
    reset-password               Resets a user's password
    restart-jellyfin             Restarts Jellyfin
    revoke-admin                 Revokes admin rights from the specified user
    scan-library                 Start a library scan
    server-info                  Displays the server information
    show-log                     Displays the requested logfile
    shutdown-jellyfin            Shuts down Jellyfin
    update-users                 Mass update users in the supplied file
```

## Installation

**Note:** All installation instructions assume the end-user can handle adding the application to their user's PATH.

### Building From Source

Currently built with rustc 1.68.0. If building on a Linux machine, you may need to install openssl-devel.

```
git clone <git location>
cd jellyroller
cargo build
```

### Initial Configuration

When running JellyRoller for the first time, you will be prompted to configure against your Jellyfin instance. You will be prompted for various items which are described below.
| Prompt | Description |
| ------------- | ------------- |
| Please enter your Jellyfin URL: | The URL to your Jellyfin instance. Depending on your setup, you may need to provide the port. Examples include http://myjellyfin.lab or http://localhost:8096. |
| Please enter your Jellyfin username: | Username with admin rights that JellyRoller will use to execute commands. |
| Please enter your Jellyfin password: | Password associated with the username being used. |

### Downloading Release

See Releases for binaries. I can currently supply builds for x86_64 Windows and x86_64 Linux. Please open an issue if you would like to request an additional format.

## Roadmap

Please open issues for feature requests or enhancements.

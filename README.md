# wireguard-manager

WireGuard administration and user management. Developed weekly live at [The Alt-F4 Stream](https://www.twitch.tv/thealtf4stream "The Alt-F4 Stream") on Twitch.

## Purpose

At the time of creating, there was no already largely used WireGuard admin tools that allowed you to maintain configuration state AND provide downloadable configurations to clients via some kind of central location.

This project is aimed to solve that probem.

## Roadmap

#### PROBLEMS:
  - As a admin, I would like to be able to add or remove users to my WG dynamically
    - Requires add or removing client keys from state
    - Requires an update to the "*.conf" file (server configuration)
    - Requires a restart or update to the wireguard interface running
  - As a user, I would like to be able to download my (and only my) WG configuration
    - Requires user to authenticate
    - Requires user to access download route -> /configs/<id>

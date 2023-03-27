<p align="center">
	<a aria-label="rock.ac logo" href="https://rockac.stalker-rp.net">
		<img src="https://rockac.stalker-rp.net/application/public/img/github-logo-dark.png" width="420" />
	</a>
</p>

<p align="center">
	<em>Rock Anti-cheat</em>
</p>

<p align="center">
	<a href="https://github.com/rock-ac/docs">
		<img src="https://img.shields.io/badge/Docs-9083D2?logoColor=9083D2" />
	</a>
</p>

<p align="center">
	Multicomplex SA:MP anti-cheat system.
</p>

<p align="center">
	<a href="https://rockac.stalker-rp.net"><strong>rock.ac »</strong></a>
</p>

<hr>

# Rock Anti-Cheat Server Plugin

This monorepo contains the server plugin for Rock Anti-Cheat. Developed in Rust to identify (and partially control, such as blocking) users in the server (analytic) anticheat system for the SA-MP server.

## Structure

-   `src/` Source code.

## Tools

-   [Rust](https://www.rust-lang.org/)

## Compiling

Install [Rust](https://www.rust-lang.org/) if it is not already installed

### Linux

```
cargo build --target=i686-unknown-linux-gnu
```

### Windows

```
cargo +nightly-i686 build --target=i686-pc-windows-gnu
```

## How to use

-   (Only for Linux) Install the necessary apt-get packages to build the project

```bash
sudo dpkg --add-architecture i386
sudo apt-get update
sudo apt-get install libssl1.1:i386
sudo apt-get install zlib1g:i386
```

-   Place the compiled one (.dll or .so, depending on the operating system) in the plugins folder and add the relevant one to server.cfg - plugins

```
plugins RockAnticheat.so
```

-   Set your server access token (from RockCP) when initializing game mode

```pawn
public OnGameModeInit() {
    RAC_SetAccessToken("your-token-here");
}
```

-   Set the active session status as soon as the player is logged on the server

```pawn
// Fictional callback, use/replace it with your own.
public OnPlayerLoggedIn(playerid) {
    if (!RAC_SetUserSessionStatus(player_name[playerid], true)) {
        Kick(playerid);
        return 0;
    }

    ...

    return 1;
}
```

-   And don't forget to set false as soon as a player disconnects from your server

```pawn
public OnPlayerDisconnect(playerid, reason) {
    RAC_SetUserSessionStatus(player_name[playerid], false);

    ...
}
```

## Authors

[@OZOSKO](https://github.com/0Z0SK0) - server analytical part of RockAC

[@savvin](https://github.com/savvin-dev) - server plugin for SA-MP server

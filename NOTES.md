# Hunter Modes
There will be two gamemodes to manage the Hunters:

## 1. Infection
If some prop dies it will be converted to Hunter

## 2. Teams
There are defined Hunters and Props

---

# Prop Role

## Tasks
As a prop, you should complete some tasks to be unnoticeable

### Independent Tasks (Infection & Teams Mode)
Small task that will accumulate time depending on difficult of the next task and distance.
If it is not completed you will be visible and the Hunter will be notified.

#### Accumulated time
Difficult:
- Easy: 1
- Medium: 2
- Hard: 3

`time = Difficult * 10 + Distance`

---

# Network configuration

The client and host read these optional environment variables at startup:

- `PROP_HUNT_SERVER_ADDR`, for example `192.168.1.20:6767`
- `PROP_HUNT_SERVER_BIND_ADDR`, for example `0.0.0.0:6767`
- `PROP_HUNT_STEAM_APP_ID`, defaulting to `480` only for `dev` builds; required for release builds

## Two-client local test

Steam must be running before starting the game. Build once from the repository root:

```sh
cargo build --features dev
```

Start the first instance and choose **Host** in the pause menu:

```sh
./target/debug/prop-hunt
```

Start a second instance in another terminal, choose **Connect**, and then choose **Resume** in both windows. The default address is `127.0.0.1:6767`.

For a client connecting to another computer, set the server address before launching it:

```sh
PROP_HUNT_SERVER_ADDR=192.168.1.20:6767 ./target/debug/prop-hunt
```

### Coop tasks (Teams Mode)
Big tasks that needs to be completed by two or more props.
It will spawn randomly and will have a limited completion time based on difficult and average distance.
If it is not complete the whole team will be visible and the Hunter will be notified.

#### Duration
Difficult:
- Easy: 2
- Medium: 3
- Hard: 4

`time = Difficult * 10 + Distance`

---

# Hunter Role

## Noticed Prop
When the Prop is close (5s) to lose its task, the Hunter will see its time counter in the screen.
If some Prop has not completed its task, it will be visible through walls for a short time (5s) and a sound will be emitted from its position.

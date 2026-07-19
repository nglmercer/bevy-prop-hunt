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

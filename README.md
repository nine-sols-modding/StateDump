# State Dump

## Attack Logic

Attacks are controlled by the following components
- Monsters have attack sensors, which check if the player is an area and certain conditions are met (e.g. player is attacking).
These are checked in `Engaging`, `LookAround`, `OutOfReach`, `Panic`, `PreAttack`, `RunAway`, `Wandering` and `WanderingIdle`.
- There is a queue of upcoming events which is queried after the previous chain finishes.
- Attack States have a clip of animation events, which are handled like this:
  - `LinkMove`
    - If it's the first time in the state, add `Initial`
    - For each `Weight`, add n copies of the attack into the queue. Either in random order (shown as dotted lines) or ordered.
    - Add all `QueueEnd` to the queue in randomized order.
  - `LinkMoveNextBreak` -> `if rand() > 0.5: LinkMove`
  - `LinkMovePhase2` -> `if phase == 2: LinkMove`
  - `LinkMovePhase3` -> `if phase == 3: LinkMove`
  - `LinkMoveInterrupt` -> TODO `currentInterruptLinkMoveSet`
  - `Done` -> `Done`
    - auto flip/auto turn
  - `CheckPlayerTooClose` -> `if dist(player, monster) < 50): go to TooCloseTransitionState` 
  - `InterruptRandomCheck` -> `if rand() > interruptChange: Done`
  - `InterruptFacing_TargetLeft` -> `if monster facing away -> LinkMove; else go to ExitState`
  - `InterruptFacing_TargetRight` -> `if monster facing player -> go to ExitState`
  - `InterruptIfTooFar` -> `if distX(player, monster) > TooFarValue: go to Engaging`
  - `InterruptIfOnSlope` -> `if onSlope > TooFarValue: go to Engaging`
  - `ForceFacing[Left,Right]`
  - `CheckFacingInverse` -> `face not to player`
  - `Evaluate` stuff
  - `{Enable,Disable}Gravity`
  - `Shoot1,Shoot2`
  - `Teleport`


linkMoveGroupIndex:
- reset on state enter
- incremented in linkmove

linkmove:
- queue current groupingNodes
  - if undefined clear + repeat

GroupingNodes
groupindex++

## Common Bosses

A more complete (but not exhaustive, ping me) list can be found at [ALL.md](./ALL.md).

### Attacks/Boss/Eigong

<picture>
<img alt="Attacks/Boss/Eigong" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Eigong/data.svg">
</picture>

### Attacks/Boss/Fuxi

<picture>
<img alt="Attacks/Boss/Fuxi" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Fuxi/data.svg">
</picture>

### Attacks/Boss/Goumang

<picture>
<img alt="Attacks/Boss/Goumang" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Goumang/data.svg">
</picture>

### Attacks/Boss/Ji

<picture>
<img alt="Attacks/Boss/Ji" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Ji/data.svg">
</picture>

### Attacks/Boss/Ji-Altar_Health_Drop

<picture>
<img alt="Attacks/Boss/Ji-Altar_Health_Drop" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Ji-Altar_Health_Drop/data.svg">
</picture>

### Attacks/Boss/Jiequan

<picture>
<img alt="Attacks/Boss/Jiequan" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Jiequan/data.svg">
</picture>

### Attacks/Boss/Ji-LaserAltar_Circle

<picture>
<img alt="Attacks/Boss/Ji-LaserAltar_Circle" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Ji-LaserAltar_Circle/data.svg">
</picture>

### Attacks/Boss/Lady Ethereal

<picture>
<img alt="Attacks/Boss/Lady Ethereal" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Lady Ethereal/data.svg">
</picture>

### Attacks/Boss/Nuwa

<picture>
<img alt="Attacks/Boss/Nuwa" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Nuwa/data.svg">
</picture>

### Attacks/Boss/Sky Rending Claw

<picture>
<img alt="Attacks/Boss/Sky Rending Claw" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Sky Rending Claw/data.svg">
</picture>

### Attacks/Boss/Yingzhao

<picture>
<img alt="Attacks/Boss/Yingzhao" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/Yingzhao/data.svg">
</picture>

### Attacks/Boss/伏羲 Variant

<picture>
<img alt="Attacks/Boss/伏羲 Variant" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/伏羲 Variant/data.svg">
</picture>

### Attacks/Boss/伏羲_新

<picture>
<img alt="Attacks/Boss/伏羲_新" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Boss/伏羲_新/data.svg">
</picture>

### Attacks/Minion/Goumang-BossZombieHammer

<picture>
<img alt="Attacks/Minion/Goumang-BossZombieHammer" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Minion/Goumang-BossZombieHammer/data.svg">
</picture>

### Attacks/Minion/Goumang-BossZombieSpear

<picture>
<img alt="Attacks/Minion/Goumang-BossZombieSpear" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Minion/Goumang-BossZombieSpear/data.svg">
</picture>

### Attacks/Minion/Goumang-ZombieBone

<picture>
<img alt="Attacks/Minion/Goumang-ZombieBone" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Minion/Goumang-ZombieBone/data.svg">
</picture>

### Attacks/Minion/Ji-BlackHole

<picture>
<img alt="Attacks/Minion/Ji-BlackHole" src="https://raw.githubusercontent.com/nine-sols-modding/StateDump/main/Attacks/Minion/Ji-BlackHole/data.svg">
</picture>

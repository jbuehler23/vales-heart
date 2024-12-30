# Vale's Heart

A top-down action RPG built with the Bevy game engine.

## Directory Structure
```
src/
├── components/      # Core game components
├── plugins/        # Plugin implementations
├── resources/      # Game resources and states
└── systems/        # Game systems
```

## Current Features
- [x] State Management (Loading, ClassSelection, Playing, Paused)
- [x] Three Playable Classes
  - Warrior (Melee Combat)
  - Archer (Ranged Combat)
  - Mage (Spell Combat)
- [x] Basic Movement System
- [x] Combat System
- [x] Pause Menu System

## Development Roadmap

### Phase 1: Combat Enhancement
- [ ] Melee Combat
  - Swing animations
  - Hit detection
  - Damage feedback
- [ ] Ranged Combat
  - Projectile aiming
  - Arrow trajectory
  - Hit effects
- [ ] Magic System
  - Spell effects
  - Area damage
  - Status effects

### Phase 2: Enemy System
- [ ] Enemy Types
- [ ] AI Behavior
- [ ] Spawning System
- [ ] Drop System

### Phase 3: Core Systems
- [ ] Health/Stats
- [ ] Inventory
- [ ] Equipment
- [ ] Progression

## Getting Started

### Prerequisites
```bash
rustup update
rustup target add x86_64-pc-windows-msvc
```

### Installation
```bash
git clone https://github.com/username/vales-heart.git
cd vales-heart
cargo build
cargo run
```

### Development Requirements
- Rust 1.75+
- Cargo
- IDE: Visual Studio Code
- Extensions:
  - rust-analyzer
  - CodeLLDB

## Controls
| Action | Key |
|--------|-----|
| Move | WASD/Arrows |
| Attack | Space |
| Pause | Escape |

## Dependencies
```toml
[dependencies]
bevy = "0.15"
bevy_rapier2d = "0.22"
```

## Contributing
1. Fork repository
2. Create feature branch
3. Commit changes
4. Open pull request

## License
MIT License
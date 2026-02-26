## [0.0.4] - 2026-02-26

### 💼 Other

- *(deps)* Bump bytes from 1.11.0 to 1.11.1 (#1)

### ⚙️ Miscellaneous Tasks

- *(dev)* Update dev-container
- *(ci)* Add copr release action
## [0.0.3] - 2026-02-21

### 🚀 Features

- *(daemon)* Add state module
- *(daemon)* Add virtual display info to status

### 🚜 Refactor

- Changed info to status
- *(daemon)* Rename gpu_info file to status
- *(logging)* Use logging as a module

### 📚 Documentation

- *(readme)* Add readme

### ⚙️ Miscellaneous Tasks

- *(release)* 0.0.3
## [0.0.2] - 2026-02-14

### 🐛 Bug Fixes

- *(daemon)* Static load edid
- *(daemon)* Service start on boot

### ⚙️ Miscellaneous Tasks

- *(logging)* Don't log logging info on release builds
- *(release)* 0.0.2
## [0.0.1] - 2026-02-14

### 🚀 Features

- *(daemon)* Added request module
- *(cli)* Added daemon command
- Added stop request + display enable
- Display info cli and daemon + enable / disable cli
- Added enable and disable commands

### 💼 Other

- Initial commit
- *(rpm)* Added rpm build
- *(release)* Release pipeline for copr
- *(copr)* Source cargo env before build
- *(release)* Add development tools for copr
- *(release)* Add leading slash in %files for copr
- *(release)* Add systemd-rpm-macros for copr

### ⚙️ Miscellaneous Tasks

- *(init)* Initial poc
- *(init)* Daemon cli basic setup
- *(dev)* Try to stop daemon after debugging
- *(release)* 0.0.1

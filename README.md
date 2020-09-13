# rustychip
Chip8 Emulator in Rust.

# Setup

## Dependencies

* Bazel
* Libraries
  * libx11-dev
  * libxext-dev
  * libxrandr-dev
  * libxrender-dev
  * libgl-dev
  * libglu-dev
  * libasound-dev
  * libalsaplayer-dev

### Installation on Ubuntu

```bash
# Install NPM
sudo apt-get install npm
# Install Bazel
sudo npm install -g @bazel/bazelisk

# Installing external dependencies.
sudo apt-get install -y \
    libsdl2-dev
```
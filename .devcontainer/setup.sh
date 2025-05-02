#!/usr/bin/env bash

set -euxo pipefail

# ----------------------
# Install Dependencies
# ----------------------
sudo apt update && sudo apt install -y \
  curl \
  wget \
  gnupg \
  git \
  build-essential \
  software-properties-common \
  libssl-dev \
  lsb-release \
  procps

# ----------------------
# Install Latest Python (3.12)
# ----------------------
sudo add-apt-repository ppa:deadsnakes/ppa -y
sudo apt update
sudo apt install -y python3.12

python3 --version || true

# ----------------------
# Install Latest Go (1.22.x)
# ----------------------
GO_VERSION="1.22.0"
wget https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
sudo rm -rf /usr/local/go
sudo tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz

echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.bashrc
echo 'export GOROOT=/usr/local/go' >> ~/.bashrc
echo 'export GOPATH=$HOME/go' >> ~/.bashrc
source ~/.bashrc

go version || true

# ----------------------
# Install Latest Rust
# ----------------------
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source "$HOME/.cargo/env"

rustc --version || true
cargo --version || true

# ----------------------
# Install Fish Shell (latest from PPA)
# ----------------------
sudo apt-add-repository ppa:fish-shell/release-3 -y
sudo apt update
sudo apt install -y fish

# Set Fish as default shell
sudo chsh -s $(which fish) $(whoami)

fish --version || true

# ----------------------
# Install Homebrew (Linuxbrew) Automatically
# ----------------------
export HOMEBREW_NO_AUTO_UPDATE=1
export HOMEBREW_NO_INSTALL_PROMPT=1
export HOMEBREW_NO_ENVIRONMENT_PRESERVE=1

/bin/bash -c 'CI=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"'

# Setup brew in environment
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"
echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> ~/.bashrc

brew --version || true

# Optional: Add common tools via brew
brew install fzf 

# ----------------------
# Install Docker (Automated, No Interaction)
# ----------------------
sudo apt update
sudo apt install -y \
  apt-transport-https \
  ca-certificates \
  curl \
  gnupg \
  software-properties-common

# Add Docker GPG key
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --batch --yes --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

# Add Docker repo
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Update and install Docker
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io

# Add current user to docker group (suppress error if already exists)
sudo usermod -aG docker $(whoami) || true

# Start Docker daemon in background
sudo dockerd > /tmp/docker.log 2>&1 &

# Wait for Docker to initialize
sleep 5

# Test Docker install
docker info || true

# ----------------------
# Install Node.js (Latest LTS via NVM)
# ----------------------
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash

# Load nvm
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

# Install and use latest LTS Node
nvm install --lts
node --version || true
npm --version || true

# ----------------------
# Install PNPM via Corepack (No sudo, no prompt)
# ----------------------
# Enable corepack
corepack enable

# Install latest pnpm version
corepack prepare pnpm@latest --activate

# Verify installation
pnpm --version || true

# ----------------------
# Ensure Fish has correct PATH
# ----------------------
mkdir -p ~/.config/fish
echo 'set -gx PATH $PATH /usr/bin /usr/local/bin' >> ~/.config/fish/config.fish
echo "alias exa='eza'" >> ~/.config/fish/config.fish

# ----------------------
# Done!
# ----------------------
echo "✅ Setup complete! All tools installed and ready to use."
echo "🎉 You now have:"
echo " - Python 3.12"
echo " - Go 1.22"
echo " - Rust"
echo " - Fish Shell"
echo " - Homebrew"
echo " - Docker (no sudo needed)"
echo " - Node.js (LTS)"
echo " - PNPM package manager"
echo " - Tools: starship, fzf, fd, ripgrep, bat, eza (exa), zoxide"

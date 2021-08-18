# Install Backupper
# Only for linux and mac

echo "Building project"
# first build project
cargo build --release

echo "Installing Bin"
# And copy bin file to /bin folder
sudo cp ./target/release/backupper /bin

echo "Installed"

# The install script for asciisavers

# Clone the repo
git clone https://github.com/coderspark/asciisavers/

cd asciisavers
# Compile
cargo build --release
# Move to binaries
mv target/release/asciisavers ~/.local/bin/

# Clean up
cd ..
rm -rf asciisavers

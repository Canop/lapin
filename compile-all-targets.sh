 
# WARNING: This script is NOT meant for normal installation, it's dedicated
# to the compilation of all supported targets. This is a long process and
# it involves specialized toolchains.
# For usual compilation do
#     cargo build --release

# clean previous build
echo "cleaning build"
rm -rf build
mkdir build

# build the linux version
echo "compiling the linux version"
cargo build --release
strip target/release/lapin
mkdir build/x86_64-linux/
cp target/release/lapin build/x86_64-linux/

# build the windows version
# You need first to install the proper cargo toolchain:
# rustup target add x86_64-pc-windows-gnu
echo "compiling the Windows version"
cargo build --target x86_64-pc-windows-gnu --release
mkdir build/x86_64-pc-windows-gnu/
cp target/x86_64-pc-windows-gnu/release/lapin.exe build/x86_64-pc-windows-gnu/


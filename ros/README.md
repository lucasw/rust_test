

from https://github.com/lucasw/rust_test/issues/2

```
cd ~/own/src/rust/
git clone https://github.com/AndrewGaspar/corrosion.git
# Optionally, specify -DCMAKE_INSTALL_PREFIX=<target-install-path>. You can install Corrosion anyway
mkdir build_corrosion
cd build_corrosion
cmake -S../corrosion -Bbuild -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
cmake --install build --config Release --prefix $HOME/other/install
```

```
PATH=$PATH:$HOME/other/install/Corrosion
```

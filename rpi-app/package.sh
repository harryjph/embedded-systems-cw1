#!/usr/bin/env bash

mkdir -p build/DEBIAN
mkdir -p build/usr/bin
mkdir -p build/etc/rpi-app
mkdir -p build/etc/systemd/system

cargo build --release

echo "url = \"http://es1-nodeapi.harryphillips.co.uk:81\"" > build/etc/rpi-app/config.toml
cp target/arm-unknown-linux-musleabihf/release/rpi-app build/usr/bin/
cp .debian/rpi-app.service build/etc/systemd/system/

cp .debian/control build/DEBIAN/control
cp .debian/postinst.sh build/DEBIAN/postinst
cp .debian/prerm.sh build/DEBIAN/prerm

chmod -R 775 build/

# Build using XZ compression
dpkg-deb -Zxz --build build
mv build.deb rpi-app.deb

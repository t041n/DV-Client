#!/usr/bin/env bash
set -e

export ANDROID_HOME=/home/syn/android-sdk
export ANDROID_SDK_ROOT=/home/syn/android-sdk
export NDK_HOME=/home/syn/android-sdk/ndk/26.3.11579264
export JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
export PATH=/home/syn/.bun/bin:/home/syn/.cargo/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$JAVA_HOME/bin:$PATH

echo "=== Android Build Environment ==="
echo "bun: $(which bun)"
echo "cargo: $(which cargo)"
echo "java: $(which java)"
echo "sdkmanager: $(which sdkmanager)"
echo "ndk clang: $(which aarch64-linux-android24-clang)"

cd /mnt/d/projects/dv-client/frontend
bun run tauri android build --debug --target aarch64

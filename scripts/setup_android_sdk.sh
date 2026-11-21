#!/usr/bin/env bash
set -e

export ANDROID_HOME=/home/syn/android-sdk
export ANDROID_SDK_ROOT=/home/syn/android-sdk
export JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$JAVA_HOME/bin:$PATH

echo "Accepting licenses..."
/usr/bin/yes | sdkmanager --licenses > /dev/null 2>&1 || true

echo "Installing Android platform-tools, platform 34, build-tools 34.0.0, and NDK..."
sdkmanager "platform-tools" "platforms;android-34" "build-tools;34.0.0" "ndk;26.3.11579264"

echo "Configuring environment in ~/.bashrc..."
if ! grep -q "ANDROID_HOME" /home/syn/.bashrc; then
  echo 'export ANDROID_HOME=/home/syn/android-sdk' >> /home/syn/.bashrc
  echo 'export ANDROID_SDK_ROOT=/home/syn/android-sdk' >> /home/syn/.bashrc
  echo 'export NDK_HOME=/home/syn/android-sdk/ndk/26.3.11579264' >> /home/syn/.bashrc
  echo 'export JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64' >> /home/syn/.bashrc
  echo 'export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$NDK_HOME:$JAVA_HOME/bin:$PATH' >> /home/syn/.bashrc
fi

echo "Android SDK setup complete!"

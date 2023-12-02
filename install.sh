#!/usr/bin/bash

cargo install --path=. --force && \
mkdir -p ~/.config/systemd/user && \
cp ./alacritty_startup.service ~/.config/systemd/user/ && \
systemctl --user daemon-reload && \
systemctl --user enable alacritty_startup.service && \
systemctl --user start alacritty_startup.service


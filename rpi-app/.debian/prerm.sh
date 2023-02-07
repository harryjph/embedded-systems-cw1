#!/usr/bin/env bash

systemctl stop rpi-app || true
systemctl disable rpi-app || true

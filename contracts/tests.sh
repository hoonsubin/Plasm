#!/usr/bin/env bash

cd wbalances
cargo contract build
cargo test
cd ../payout/
cargo contract build
cargo test
#! /usr/bin/env sh

rm -r $(pwd)/wow_login_messages/src/logon || true

rm -r $(pwd)/wow_vanilla_messages/src/world/version_1_12 || true
rm -r $(pwd)/wow_vanilla_messages/src/world/version_1_2 || true
rm $(pwd)/wow_vanilla_messages/src/world/mod.rs || true

rm -r $(pwd)/wowm_language/src/docs || true

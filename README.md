SETUP - 
1- brew install git unzip gzip xz
2- curl -fsSL https://moonrepo.dev/install/proto.sh | bash
3- source ~/.zshrc
4- proto install moon


To run-
1- to get an overview of the repo and projects. run - moon query projects
2- to install, compile rust, and test run -  moon run @stitchmate/backend/user:test-common -- --lib domain

to run an app -
SM__REPOSITORY__CONFIG__PASSWORD="password" cargo run --release | jq .
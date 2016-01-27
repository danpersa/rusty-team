# Rusty Team

This is a mock Team Service.

We provide an member username with a specific format as an url parameter. This param will be parsed and data from it extracted.

For example:

    curl -v http://localhost:6768/api/teams\?member\=user~1

Will return:

    [{"id":"team1","type":"official"},{"id":"other_team1","type":"virtual"}]


# Docker Machine with Virtual Box

    VBoxManage controlvm "default" natpf1 "tcp-port6768,tcp,,6768,,6768"

# Init swarm

Build docker images for test-api and test-client.
Create network : `docker network create -d overlay --attachable swarm_network`.
To run a stack: `docker stack deploy --compose-file dockerswarm_{service}.yml {stack_name}`.
To remove stack: `docker stack rm {stack_name}`.


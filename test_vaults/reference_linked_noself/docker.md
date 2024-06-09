---
bad_links:
aliases: []
tags: [networks, operatingsystems]
---
# Docker

## Enter Its Shell

```
docker ps # Get its name
docker exec -it <name> sh
```

## Info

Docker is an open-source platform that automates the deployment, scaling, and management of applications. It does this by using containerization, a lightweight form of virtualization.

Containerization involves encapsulating an application and its dependencies into a container, which can then be run on any system that supports Docker. This ensures that the application behaves the same way regardless of where it is run, eliminating the "it works on my machine" problem.

Docker containers are created from Docker images, which are essentially snapshots of a container's file system. Docker images are built from Dockerfiles, which are scripts containing instructions on how to build the image.

Docker uses a client-server architecture. The Docker client communicates with the Docker daemon which does the heavy lifting of building, running, and managing Docker containers. Both the client and the daemon can run on the same host, or the client can communicate with a remote daemon

Docker also provides a networking model that allows containers to communicate with each other and with the outside world. It supports different types of networks, each suited to specific use cases.

Docker Compose is a tool for defining and running multi-container Docker applications. With Compose, you use a YAML file to configure your application's services, and then with a single command, you can create and start all the services.

Docker Swarm is a native clustering and orchestration solution for Docker. It turns a pool of Docker hosts into a single, virtual Docker host, and allows you to manage a swarm of Docker nodes as a single entity.

Docker also involves some important concepts from computer science, such as process isolation, virtualization, and the client-server model.

> For more information, you can refer to the [official Docker documentation](https://docs.docker.com/), or this [Google search for Docker](https://www.google.com/search?q=Docker).
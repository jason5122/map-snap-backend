# MapSnap Backend

A simple Rust server/microservice example for [Docker's Rust Language Guide](https://docs.docker.com/language/rust/).

## Figuring out AWS and Docker

### Fargate

- [Useful tutorial](https://www.padok.fr/en/blog/ecs-fargate-deployment-app)

### VPC

- The default security group has no inbound rules. You need to create a security group and add inbound rules for specific ports if you want the public IP to be reachable. [StackOverflow](https://stackoverflow.com/a/71950809/14698275)

### ECR

- A **repository** holds all the versions of the same image. The repository name should be the image name.

### Docker

- Ensure we build for linux/amd64 (x86) on Apple Silicon.

### CloudWatch

- Used `/ecs/map-snap-backend` log group to view "exec format error" message.
- App Runner (`/aws/apprunner/map-snap-backend/...`) _does not_ output detailed error messages! [GitHub issue](https://github.com/aws/apprunner-roadmap/issues/110)

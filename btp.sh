# todo: run off tags, not latest
docker build . -t local:latest
docker tag local:latest nkuila/facebook-contrabanned:latest
docker push nkuila/facebook-contrabanned:latest
docker build . -t local

docker stop contrabanned
docker rm contrabanned
docker run -p 8000:8000 --name contrabanned -d local
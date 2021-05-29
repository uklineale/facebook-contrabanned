docker stop contra
docker rm contra
docker run -p 8000:8000 --name contra -d local
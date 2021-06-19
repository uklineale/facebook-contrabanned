sudo docker stop contra
sudo docker rm contra
sudo docker run -p 8000:8000 --name contra --pull always -d nkuila/facebook-contrabanned:latest
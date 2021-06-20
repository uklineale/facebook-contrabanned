FROM python:3.8

COPY .. /src/
RUN pip3 install -r /src/requirements.txt

WORKDIR /src/server
ENTRYPOINT gunicorn -b 0.0.0.0:8000 wsgi:app

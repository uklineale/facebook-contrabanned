from main import app

if __name__ == '__main__':
    # Port doesn't mattter, gets overidden by gunicorn in the container
    app.run(host='0.0.0.0', port=4001)
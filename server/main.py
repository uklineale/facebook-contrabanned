from flask import Flask, request, redirect
import uuid

app = Flask(__name__)
redirect_sets = {}

class RedirectSet:
    def __init__(self, real_url, fake_url):
        self.id = str(uuid.uuid4())
        self.real_url = real_url
        self.fake_url = fake_url

def is_facebook(r):
    isCrawler = False
    userAgent = request.headers['User-Agent']
    print(userAgent)
    if 'facebook' in userAgent:
        isCrawler = True

    return isCrawler
@app.route('/')
def home():
    return "Hi, welcome to Chili's!"

@app.route('/redirects/<id>', methods=['GET'])
def handle_redirect(id):
    try:
        rs = redirect_sets[id]
    except KeyError:
        return 400

    if is_facebook(request):
        return redirect(rs.fake_url)
    else:
        return redirect(rs.real_url)

@app.route('/redirects', methods=['POST'])
def create_redirect_set():
    rs = RedirectSet(request.form['real_url'],
                     request.form['fake_url'])
    redirect_sets[rs.id] = rs
    return '''Created RedirectSet.
           ID is %s
           Real URL: %s
           Fake URL: %s
           ''' % (rs.id, rs.real_url, rs.fake_url)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=7000)
from flask import Flask, request, redirect, render_template
import dao

app = Flask(__name__)

def is_facebook(r):
    isCrawler = False
    userAgent = r.headers['User-Agent']
    print(userAgent)
    if 'facebook' in userAgent:
        isCrawler = True

    return isCrawler

@app.route('/')
def home():
    return render_template('index.html', title='Contrabanned')

@app.route('/redirects/<id>', methods=['GET'])
def handle_redirect(id):
    redirect_set = dao.get(id)
    if redirect_set is None:
        # Todo: Friendlier 404 page
        return "Item not found", 404

    if is_facebook(request):
        return redirect(redirect_set.fake_url)
    else:
        return redirect(redirect_set.real_url)

@app.route('/redirects', methods=['POST'])
def create_redirect_set():
    real_url = request.form['real_url']
    fake_url = request.form['fake_url']

    if real_url is None or real_url == '' or \
            fake_url is None or fake_url == '':
        return "Bad request", 400

    id = dao.insert(real_url, fake_url)
    return '%s' % (id)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=7000)

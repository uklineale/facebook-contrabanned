from flask import Flask, request, redirect, render_template
import dao

app = Flask(__name__)

def is_facebook(r):
    isCrawler = False
    userAgent = request.headers['User-Agent']
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
    id = dao.insert(request.form['real_url'],
                     request.form['fake_url'])
    print(id)
    return 'Your link to use is /redirects/%s' % (id)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=7000)

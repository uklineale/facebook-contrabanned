const website = 'http://34.217.175.63/:8000/redirects'
async function createRedirectSet() {
    var outputHeader = document.getElementById('output-header')
    var outputField = document.getElementById('output')
    var realUrl = document.getElementById('real-url').value;
    var fakeUrl = document.getElementById('fake-url').value;
    var data = 'real_url=' + realUrl + '&fake_url=' + fakeUrl
    console.log(data)

    fetch(website, {
        method: 'POST',
        mode: 'cors',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: data
    })
    .then(res => res.text())
    .then(redirectId => {
        outputHeader.innerHTML = 'The link to your Redirect is:'
        outputField.innerHTML = website + '/' + redirectId
    })
}
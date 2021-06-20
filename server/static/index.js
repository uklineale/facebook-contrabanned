const endpoint = location.protocol + '//' + location.hostname +
    (location.port ? ':'+location.port: '')+ '/redirects'
async function createRedirectSet() {
    var outputHeader = document.getElementById('output-header')
    var outputField = document.getElementById('output')
    var realUrl = document.getElementById('real-url').value;
    var fakeUrl = document.getElementById('fake-url').value;
    var data = 'real_url=' + realUrl + '&fake_url=' + fakeUrl
    console.log(data)
    console.log(endpoint)

    fetch(endpoint, {
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
        if (redirectId === "Bad request") {
            outputHeader.innerHTML = 'Invalid input provided.'
            outputField.innerHTML = ''
        } else {
            outputHeader.innerHTML = 'The link to your Redirect is:'
            outputField.innerHTML = endpoint + '/' + redirectId
        }
    })
}
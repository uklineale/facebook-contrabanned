function createRedirectSet() {
    var realUrl = document.getElementById('real-url').value;
    var fakeUrl = document.getElementById('fake-url').value;
    var data = 'real_url=' + realUrl + '&fake_url=' + fakeUrl
    console.log(data)

    // Todo: get request working
    var request = new XMLHttpRequest();
    request.open('POST', 'http://localhost:7000/redirects', true);
    request.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');

    request.onreadystatechange = function() {
      if (this.readyState == XMLHttpRequest.DONE && this.status == 200) {
        console.log('succeed');
        myresponse.value = request.responseText;
      } else {
        console.log('server error');
      }
    };

    request.onerror = function() {
      console.log('something went wrong');
    };

    request.send(data);
}
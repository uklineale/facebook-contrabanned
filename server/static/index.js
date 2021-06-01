function createRedirectSet() {
    var mydata = document.getElementById('mydata');
    var myresponse = document.getElementById('myresponse');
    var data = {
        'real_url' : 'real URL',
        'fake_url' : 'fake URL'
    }

    // Todo: get request working
    var request = new XMLHttpRequest();
    request.open('POST', 'http://localhost:7000/redirects/', true);
    request.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded; charset=UTF-8');

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
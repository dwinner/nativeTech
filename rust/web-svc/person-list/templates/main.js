function getPage(uri) {
    let xmlHttpRequest = new XMLHttpRequest();
    xmlHttpRequest.onreadystatechange = function () {
        if (this.readyState === 4 && this.status === 200) {
            document.getElementById('body')
                .innerHTML = xmlHttpRequest.responseText;
        }
    };

    xmlHttpRequest.open('GET', uri, true);
    xmlHttpRequest.send();
}

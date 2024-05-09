const DocsSources = ( e => {

    let createPdfLinksList = e => {
        let pdfLinksList = document.createElement('div');
        pdfLinksList.className = 'pdfLinksList';
        pdfLinksList.id = 'pdfLinksList';

        let paragraph = document.createElement('p');
        let icon = document.createElement('i');
        icon.className = 'fa-solid fa-paperclip';

        paragraph.appendChild(icon);
        paragraph.appendChild(document.createTextNode('Documents:'));
        pdfLinksList.appendChild(paragraph);

        document.body.appendChild(pdfLinksList);
    };

    let checkDocsUrlExt = url => {
        return !!(url.endsWith('.pdf') || url.endsWith('.docx') || url.endsWith('.epub') || url.endsWith('.mobi') || url.endsWith('.doc') || url.endsWith('.rst') || url.endsWith('.yml') || url.endsWith('.yaml') || url.endsWith('.json') || url.endsWith('.toml') || url.endsWith('.7z') || url.endsWith('.zip') || url.endsWith('.rar') || url.endsWith('.tar') || url.endsWith('.tar.gz') || url.endsWith('.gz') || url.endsWith('.bin') || url.endsWith('.img'));
    };

    let listDocs = e => {
        let docsList = [];

        Array.from(document.querySelectorAll('a')).forEach( link => {
            let url = link.href;

            if (checkDocsUrlExt(url)) {
                let fileName = url.split('/').pop();

                if (!docsList.some(item => item.name === fileName)) {
                    docsList.push({
                        url: url,
                        name: fileName,
                    });
                }
            }
        });

        let ul = document.createElement('ul');
        let pdfLinksListDiv = document.getElementById('pdfLinksList');

        docsList.forEach( doc => {
            let li = document.createElement('li');
            let a = document.createElement('a');

            a.href = doc.url;
            a.target = '_blank';
            a.textContent = doc.name;
            
            li.appendChild(a);
            ul.appendChild(li);
        });

        pdfLinksListDiv.appendChild(ul);
    };

    createPdfLinksList();
    listDocs();
    
})();

const DocsSources = ( e => {

    let nameElementRoot = 'docsSourceList';
    let classNameElementRoot = 'paimon-box';
    let classesElementEffect = ['animate__animated', 'animate__bounce'];

    let getDomain = url => {
        var urlObj = new URL(url);
        return urlObj.hostname;
    };

    let createPdfLinksList = e => {
        let pdfLinksList = document.createElement('div');
        pdfLinksList.className = classNameElementRoot;
        pdfLinksList.id = nameElementRoot;

        let paragraph = document.createElement('p');
        let icon = document.createElement('i');
        icon.className = 'fa-solid fa-paperclip';

        paragraph.appendChild(icon);
        let textLabel = document.createTextNode('Documents:');
        paragraph.appendChild(textLabel);
        pdfLinksList.appendChild(paragraph);

        document.body.appendChild(pdfLinksList);
    };

    let checkDocsUrlExt = url => {
        return !!(url.endsWith('.pdf') || url.endsWith('.docx') || url.endsWith('.epub') || url.endsWith('.mobi') || url.endsWith('.doc') || url.endsWith('.rst') || url.endsWith('.yml') || url.endsWith('.yaml') || url.endsWith('.json') || url.endsWith('.toml') || url.endsWith('.7z') || url.endsWith('.zip') || url.endsWith('.rar') || url.endsWith('.tar') || url.endsWith('.tar.gz') || url.endsWith('.gz') || url.endsWith('.bin') || url.endsWith('.img'));
    };

    let listDocs = e => {
        let docsList = [];

        Array.from(
            document.querySelectorAll('a')
        ).forEach( link => {
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

        if (docsList.length > 0) {
            createPdfLinksList();

            let ul = document.createElement('ul');
            let pdfLinksListDiv = document.getElementById(nameElementRoot);
    
            docsList.forEach( doc => {
                let li = document.createElement('li');
                li.className = 'pdd-left';
                
                let a = document.createElement('a');
    
                a.href = doc.url;
                a.target = '_blank';
                a.textContent = doc.name + ` (from: ${ getDomain(doc.url) })`;
                
                li.appendChild(a);
                ul.appendChild(li);
            });
    
            pdfLinksListDiv.appendChild(ul);
        }
    };

    let scrollBounceEffet = el => {
        el.classList.remove(...classesElementEffect);
        setTimeout( e => { el.classList.add(...classesElementEffect); }, 100);
    };

    let scrollTo = e => {
        let element = document.getElementById(nameElementRoot);

        if (element) {
            element.scrollIntoView({ behavior: 'smooth' });
            scrollBounceEffet(element);
        }
    };
    
    return {
        init: () => { return listDocs(); },
        scrollTo: () => { return scrollTo(); },
        getNameElementRoot: () => { return nameElementRoot; },
    };
    
})();

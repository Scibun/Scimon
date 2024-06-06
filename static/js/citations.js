const Citations = ( e => {

    let nameElementRoot = Elements.refsList;
    let classNameElementRoot = 'plugin-section';
    let headerScrollToBtn = Elements.scrollToRefsBtn;

    let Cite = require('citation-js');

    let testRegex = (value) => {
        return /^@[a-zA-Z]+\{/.test(value);
    };

    let createRefsList = e => {
        let refsList = document.getElementById(nameElementRoot);

        if (!refsList) {
            refsList = document.createElement('div');
            refsList.className = classNameElementRoot;
            refsList.id = nameElementRoot;

            let paragraph = document.createElement('p');
            let icon = document.createElement('i');
            icon.className = 'fa-solid fa-book';

            paragraph.appendChild(icon);
            let textLabel = document.createTextNode('Citations:');
            paragraph.appendChild(textLabel);
            refsList.appendChild(paragraph);

            let ul = document.createElement('ul');
            refsList.appendChild(ul);

            document.querySelector(`.${Elements.classNames.markdownBody}`).appendChild(refsList);
        }
    };

    let convertToCitation = (bibText) => {
        return new Cite(bibText).format('bibliography', {
            lang: 'en-US',
            format: 'html',
            template: 'apa',
        });
    };

    let checkExists = e => {
        let existsRefs = false;
    
        Array.from(
            document.querySelectorAll('p')
        ).forEach( citation => {
            let text = citation.textContent;
            if (testRegex(text)) { existsRefs = true; }
        });
    
        return existsRefs;
    };

    let load = e => {
        if (checkExists()) {
            createRefsList();
            
            document.querySelectorAll('p').forEach( block => {
                let text = block.textContent;
    
                if (testRegex(text)) {
                    const refsList = document.getElementById(nameElementRoot).querySelector('ul');
                    const citationItem = document.createElement('li');
                    
                    citationItem.className = 'ref';
                    citationItem.innerHTML = `
                        <i class='fa-solid fa-code hover-icon' data-bibtex='${ text }' onclick='BibTex.get(this)'></i>
                        ${ convertToCitation(text) }
                    `;
    
                    refsList.appendChild(citationItem);
                    block.remove();
                }
            });

            document.getElementById(headerScrollToBtn).style.display = 'inline';
        } else {
            document.getElementById(headerScrollToBtn).style.display = 'none';
        }
    };
    
    return {
        element: nameElementRoot,
        toggleBtn: headerScrollToBtn,
        
        init: () => { return load(); },
        scrollTo: () => { return ScrollTo.element(nameElementRoot); },
    };

})();

const Citations = ( e => {

    let nameElementRoot = 'refsList';
    let classNameElementRoot = 'plugin-section';
    let classesElementEffect = ['animate__animated', 'animate__bounce'];

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

            document.body.appendChild(refsList);
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
                        <i class='fa-solid fa-bookmark'></i>
                        ${ convertToCitation(text) }
                    `;
    
                    refsList.appendChild(citationItem);
                    block.remove();
                }
            });
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
        init: () => { return load(); },
        scrollTo: () => { return scrollTo(); },
        getNameElementRoot: () => { return nameElementRoot; },
    };

})();
